# Next-session prompt — fully verify `deserialize_5`, plus storeu/loadu axiom upstream

**Branch:** start from `agent-mlkem-serialize-1-deserialize-5-2026-05-11`
(or whatever you merged it into).
**Tip on entry:** must include commit `6f7cdd2ad`
("agent-mlkem: fully verify serialize_1 (lax → fully proven)") AND the
unstaged `BitVec.Intrinsics.fsti` change adding
`mm256_mullo_epi16_specialized4` (see "Inherited state" below).

Verify by `grep -c mm256_mullo_epi16_specialized4
fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` returning ≥2 (one
for the spec, one for the dispatch arm).

## What's done — sprint 2026-05-11

- **`serialize_1` fully verified** (commit `6f7cdd2ad`). Proof
  technique: `introduce forall ... with match i with | 0 -> assert_norm
  ... | ... | _ -> ...` over 16 explicit branches. Now documented in
  `~/.claude/skills/fstar-for-libcrux/SKILL.md` §7. The match
  destructures the Skolemized `i` into N **separate goals** *before*
  SMT, so each branch hits Z3 with one closed equality at concrete `i`.
  Use this when N ≤ 32 and the chain reduces individually per `i`.

- `BitVec.Intrinsics.fsti` already has `mm256_mullo_epi16_specialized4`
  added (currently uncommitted on this branch).  Spec:
  ```fstar
  let mm256_mullo_epi16_specialized4 (a: bit_vec 256): bit_vec 256 =
    mk_bv (fun i ->
      let nth_bit = i % 16 in
      let nth_i16 = i / 16 in
      let k = nth_i16 % 8 in
      let shift = (k % 2) * 5 + (k / 2) * 2 in
      if nth_bit >= shift then a (i - shift) else 0)
  ```
  Plus a 4th `unify_app` arm in the `mm256_mullo_epi16` tactic
  matching the multiplier `(1<<0, 1<<5, 1<<2, 1<<7, 1<<4, 1<<9, 1<<6,
  1<<11)` (repeated for the upper half).  **Decision needed**: commit
  the BitVec edit as a standalone preparation commit, or fold into the
  deserialize_5 commit.

- **`deserialize_5` is unchanged from baseline** — still
  `verification_status(panic_free)`.  The source-level revert wiped
  ~30 minutes of failed proof attempts (see "Approaches that don't
  work" below).

## Branch hygiene — mandatory

The user works in `/Users/karthik/libcrux-trait-opacify` on parallel
tasks.  **Do NOT operate on the shared worktree.** Per
`feedback_branch_means_worktree`:

```bash
git -C /Users/karthik/libcrux-trait-opacify worktree add \
    /Users/karthik/libcrux-deserialize-5-fully \
    -b agent-mlkem-deserialize-5-fully-2026-05-12 \
    agent-mlkem-serialize-1-deserialize-5-2026-05-11   # base
cd /Users/karthik/libcrux-deserialize-5-fully/libcrux-ml-kem
```

If the prior worktree
(`/Users/karthik/libcrux-serialize-1-deserialize-5`) still exists,
prompt the user before reusing it.

**`.fstar-cache` is per-worktree and starts empty.** Copy from a
sibling worktree on a same-or-similar branch:
```bash
cp -R /Users/karthik/libcrux-serialize-1-deserialize-5/.fstar-cache/checked/. \
      /Users/karthik/libcrux-deserialize-5-fully/.fstar-cache/checked/
mkdir -p /Users/karthik/libcrux-deserialize-5-fully/.fstar-cache/hints
cp -R /Users/karthik/libcrux-serialize-1-deserialize-5/.fstar-cache/hints/. \
      /Users/karthik/libcrux-deserialize-5-fully/.fstar-cache/hints/ 2>/dev/null
```

## Inherited state — `BitVec.Intrinsics.fsti` changes

If your worktree base does NOT include the `specialized4` edit,
re-create it from this diff:

```diff
@@ -157,6 +157,17 @@ let mm256_mullo_epi16_specialized3 (a: bit_vec 256): bit_vec 256 =
     if nth_bit >= shift then a (i - shift) else 0
   )

+// For deserialize_5: per-lane shift cycle of period 8 = [0;5;2;7;4;9;6;11].
+// Equivalently: shift(k) = (k % 2) * 5 + ((k % 8) / 2) * 2.
+let mm256_mullo_epi16_specialized4 (a: bit_vec 256): bit_vec 256 =
+  mk_bv (fun i ->
+    let nth_bit = i % 16 in
+    let nth_i16 = i / 16 in
+    let k = nth_i16 % 8 in
+    let shift = (k % 2) * 5 + (k / 2) * 2 in
+    if nth_bit >= shift then a (i - shift) else 0
+  )
+
 // This term will be stuck, we don't know anything about it
 val mm256_mullo_epi16_no_semantics (a count: bit_vec 256): bit_vec 256
@@ -197,6 +208,16 @@ let mm256_mullo_epi16
              then Tactics.exact (quote (mm256_mullo_epi16_specialized3 a))
              else
+               if match unify_app (quote count) (quote (fun x -> mm256_set_epi16 (x <<! (mk_i32 0) <: i16) ((mk_i16 1) <<! (mk_i32 5) <: i16)
+                                                                 ((mk_i16 1) <<! (mk_i32 2) <: i16) ((mk_i16 1) <<! (mk_i32 7) <: i16) ((mk_i16 1) <<! (mk_i32 4) <: i16) ((mk_i16 1) <<! (mk_i32 9) <: i16) ((mk_i16 1) <<! (mk_i32 6) <: i16)
+                                                                 ((mk_i16 1) <<! (mk_i32 11) <: i16) ((mk_i16 1) <<! (mk_i32 0) <: i16) ((mk_i16 1) <<! (mk_i32 5) <: i16) ((mk_i16 1) <<! (mk_i32 2) <: i16) ((mk_i16 1) <<! (mk_i32 7) <: i16)
+                                                                 ((mk_i16 1) <<! (mk_i32 4) <: i16) ((mk_i16 1) <<! (mk_i32 9) <: i16) ((mk_i16 1) <<! (mk_i32 6) <: i16) ((mk_i16 1) <<! (mk_i32 11) <: i16))) [] with
+                  | Some [x] -> unquote x = (mk_i16 1)
+                  | _ -> false
+               then Tactics.exact (quote (mm256_mullo_epi16_specialized4 a))
+               else
                  Tactics.exact (quote (mm256_mullo_epi16_no_semantics a count))
     )]result: bit_vec 256): bit_vec 256 = result
```

After applying, `rm BitVec.Intrinsics.fsti.checked` and re-make to
rebuild.  Touches one shared module — verified to not regress
`Vector.Avx2.Serialize.fst` make (rc=0, 2:19 wall on cold cache).

## Approaches that DO NOT work for `deserialize_5` (do not retry)

The `deserialize_5` ensures is `forall (i: nat{i < 256}). $result i =
...`.  **256 elements**, not 16.  The serialize_1 technique does NOT
mechanically generalize.

| Approach | Result |
|---|---|
| `assert_norm (BitVec.Utils.forall_n 256 (fun i -> ...))` | Z3 timeout, 5:52 wall, rlimit 400 burnt. Too many conjuncts for one query. |
| `match i with | 0 -> assert_norm ... | ... | 255 -> ...` | 256 explicit arms — impractical to write; F* match destructuring not tested at this scale. |
| `match (i / 16) with | k -> assert_norm (forall_n 16 (fun b -> P (k*16+b)))` | Z3 timeout. The inner `forall_n 16` over `pred (k*16+b)` doesn't reduce in F* normalizer at concrete `k`, because `pred` is a closure capturing `result`/`bytes`/`coefficients` and the inner expression `pred (k*16+b)` doesn't decompose under normalization the way concrete-`i` `pred i` does. Killed at ~5 min. |

The third one is structurally similar to what worked in serialize_1
but with an extra layer of forall_n inside.  **Don't waste time
re-trying it** — the failure mode is that `pred` can't be normalized
at symbolic `b`.

## Recommended approach: factor `deserialize_5_vec` helper

Mirror `serialize_5_vec`'s pattern (line 364 of `serialize.rs`).  The
existing `serialize_5_vec` factored the SIMD-only chain into an inner
function with its OWN ensures keyed off `forall_n 40` per half.  This
worked because:
1. The inner function's spec was tighter (40-bit slices, not 256).
2. `forall_n 40` over a SIMD chain with NO `saturate8`-style 8-way
   logic reduces well in F* normalizer.
3. The outer function then composes 2 × 40-bit slices into the final
   80-bit result via straightforward Seq operations.

For `deserialize_5`, the natural factor is at the SIMD-only inner
chain, before the final `mm256_srli_epi16<11>`:

```rust
#[inline(always)]
#[hax_lib::fstar::options("--ext context_pruning --split_queries always --z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Seq.length bytes == 10"#))]
#[hax_lib::ensures(|coefficients_pre_srli| fstar!(r#"
  forall (i: nat{i < 256}). i % 16 < 5 ==>
    $coefficients_pre_srli (i + 11) =
      bit_vec_of_int_t_array ($bytes <: t_Array _ (sz 10)) 8 ((i / 16) * 5 + i % 16)
"#))]
fn deserialize_5_vec(bytes: &[u8]) -> Vec256 {
    // body: mm_set_epi8 → from_two_si128 → shuffle_epi8 → mullo_epi16
    // (no srli)
}
```

Inside `deserialize_5_vec`'s body, discharge the ensures with a
two-half split (mirroring serialize_5):

```fstar
introduce forall (i: nat{i < 128}). i % 16 < 5 ==>
  $coefficients_pre_srli (i + 11) = bit_vec_of_int_t_array $bytes 8 ((i/16)*5 + i%16)
with assert_norm (BitVec.Utils.forall_n 128 (fun i -> ...));

introduce forall (i: nat{i < 128}). i % 16 < 5 ==>
  $coefficients_pre_srli (128 + i + 11) = bit_vec_of_int_t_array $bytes 8 (((128+i)/16)*5 + i%16)
with assert_norm (BitVec.Utils.forall_n 128 (fun i -> ...))
```

(Adjust the predicate exactly — the example above hand-waves the
half-split.  Do `forall_n 64` per quarter if 128 is too big.)

Then the outer `deserialize_5` calls `deserialize_5_vec` and the
final `mm256_srli_epi16<11>` step is straightforward to discharge
because `srli`'s spec is concrete.

**Before writing the helper, run a small experiment to confirm
`forall_n 128` (or `forall_n 64`) over the chain reduces in F*
normalizer.**  Pattern:

```fstar
hax_lib::fstar!(r#"
  introduce forall (i: nat{i < 128}). i % 16 < 5 ==>
    $coefficients_inner (i + 11) = $bytes_bits ((i/16)*5 + i%16)
  with assert_norm (BitVec.Utils.forall_n 128 (fun i ->
    i % 16 < 5 ==>
      $coefficients_inner (i + 11) = $bytes_bits ((i/16)*5 + i%16)))
"#)
```

If that times out, fall back to `forall_n 64` and split into 4
quarters (or 8 eighths).

**Critical:** verify that `mm256_mullo_epi16_specialized4` actually
matches the multiplier in `deserialize_5`.  Add a sanity check
assertion early:

```fstar
hax_lib::fstar!(r#"
  // Sanity: confirm specialized4 dispatch fires at lane 0
  assert_norm (
    let mul = BitVec.Intrinsics.mm256_mullo_epi16_specialized4 coefficients_after_shuffle in
    mul 11 = coefficients_after_shuffle 11   (* shift(lane 0) = 0 *)
  )
"#)
```

If this `assert_norm` fails, the unify pattern in BitVec.Intrinsics
isn't matching — debug the multiplier syntax in `mm256_mullo_epi16`'s
4th arm (most likely a literal-form mismatch — `mk_i16 1` vs `1s` etc.).

## Out of scope (separate sprint)

- **Stretch storeu/loadu axiom upstream** (the next-session prompt
  from sprint 2026-05-11 mentioned this).  Two `admit ()` lines in
  `serialize_11`'s `before` block (`mm256_storeu_si256_i16_post_axiom`,
  `mm256_loadu_si256_i16_post_axiom`).  Path: strengthen the val
  ensures in `crates/utils/intrinsics/src/avx2_extract.rs`'s
  `#[hax_lib::ensures(...)]` for the storeu/loadu intrinsics so the
  local axioms become consequences.  Estimate: 30 min.
- All other lax sites in the AVX2 / Neon / portable serialize files —
  separate sprints.

## Pre-session checklist

- [ ] Worktree created (per Branch hygiene).  Confirm `pwd`.
- [ ] Tip: `git log -1 --oneline` includes `6f7cdd2ad` in ancestry.
- [ ] Read `~/.claude/skills/fstar-for-libcrux/SKILL.md` — especially
      §7 "Per-i match for forall over a SIMD chain — the BV-tactic
      alternative" (the technique used in serialize_1).
- [ ] Read MEMORY.md `feedback_proof_debug_budget` — 30-60 min hard
      cap per fn.  Time budget for this sprint: 60-90 min if going
      via the `deserialize_5_vec` factor; if exceeded, document
      blocker and stop.
- [ ] Apply the `BitVec.Intrinsics.fsti` diff above if not already in
      base.  Confirm `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst`
      rc=0 with `deserialize_5` still `panic_free`.
- [ ] Run the sanity `assert_norm` on `mm256_mullo_epi16_specialized4`
      to confirm the unify pattern fires.

## Time budget per `feedback_proof_debug_budget`

- 60-90 min for `deserialize_5` via the `_vec` factor.  If the
  `forall_n N` doesn't reduce after 30 min of tweaking, **stop**.
  The likely fallback is per-quarter / per-eighth match, which is
  more code but mechanical.
- 30 min for storeu/loadu upstream (separate, optional).

## Status reports

Per `feedback_agent_status_reports`: every 15 min, append a 3-line
status to `proofs/agent-status/sprint-2026-05-12-status.md`:
- Current site (which fn, which factoring step).
- Blocker if any (specific Z3 query / lemma).
- ETA for current site.

## Key file paths quick reference

- `libcrux-ml-kem/src/vector/avx2/serialize.rs` — sprint surface.
- `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` — already has
  `mm256_mullo_epi16_specialized4` (uncommitted).
- AVX2 serialize_5_vec template: `serialize.rs` line ~364.
- AVX2 deserialize_10_vec template (different shape but same factor
  pattern): `serialize.rs` around line 626.
- Sprint 2026-05-11 retrospective:
  `proofs/agent-status/sprint-2026-05-11-status.md` (write this
  before closing the session).

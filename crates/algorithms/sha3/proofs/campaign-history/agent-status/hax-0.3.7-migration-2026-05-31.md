# hax-lib 0.3.7 migration — status / handoff (2026-05-31)

Worktree: `/Users/karthik/libcrux-sha3-hax037`, branch `sha3-hax-0.3.7-migration`,
off the squeeze4 tip `75721cd1f` (main `sha3-proofs-focused`, which is at hax-lib
0.3.6 / `integer-lemmas` and reverted clean — DO NOT commit the 0.3.7 pin there).

## Goal
Migrate sha3 proofs from hax-lib 0.3.6 (`branch=integer-lemmas`, rev `952bee0`) to
0.3.7 (`tag=cargo-hax-v0.3.7`, rev `d8b5b3d`).

## Setup done in this worktree
- `Cargo.toml`: both hax-lib deps pinned `tag = "cargo-hax-v0.3.7"`; `hpke-rs`
  commented (line 191) so cargo resolves to a SINGLE hax-lib 0.3.7 (otherwise the
  crates.io libcrux-* dev-dep chain drags in hax-lib 0.3.6 → two proof-lib trees on
  the F* include path → `Error 308` recursive-dependency cycle). Reference: the
  `user14-bridge` worktree runs clean 0.3.7 precisely because it has no hpke-rs.
- `bash hax.sh extract` run (the extraction `.fst` are gitignored, so a fresh
  worktree lacks them). Re-extraction under 0.3.7 is BYTE-IDENTICAL to 0.3.6 — the
  hax proc-macro is unchanged; only F* proof-libs differ.

## Proof-lib diff 0.3.6 → 0.3.7 (the whole story)
Only 4 files differ; `proofs/fstar/extraction` (hax support) is byte-identical:
`Core_models.Num.fst` (`impl_uN__rotate_left` `assume val` → concrete, delegating
to new `Rust_primitives.Arithmetic.rotate_left_uN` aliases), `Core_models.Array.fst`
(whitespace), `Core_models.Ops.Function.fst` (+`unfold` quals),
`Rust_primitives.Arithmetic.fsti` (+`rotate_left_uN` aliases). All additive/
cosmetic + the rotate_left-concrete change. These `unfold`/concrete edits perturb
`fuel`-driven normalization.

## Result: 154/156 modules verify under 0.3.7; 2 genuinely fail
Full-tree gate (fresh cache `/tmp/sha3-037-cache`, `make -k -j2`): 154 modules pass.
Two ROOT failures (everything else that "didn't verify" is just blocked downstream
of these):

1. **`EquivImplSpec.Sponge.Avx2.fst` — `lemma_sq_lane_avx2_eq_squeeze_state`**
   (the `sq_lane_avx2 == squeeze_state` per-byte bridge). `Error 19`,
   "could not prove post-condition — unknown because (incomplete quantifiers)".
2. **`Libcrux_sha3.Generic_keccak.Simd128.fst` — `squeeze2` composer**
   (arm64 N=2). `Error 19`, assertion failed.

### Crucial diagnosis (bisect-confirmed): these are HINT-DEPENDENT, not 0.3.7 regressions
`lemma_sq_lane_avx2_eq_squeeze_state` fails hint-free at **0.3.6 too** (fresh cache:
629/800, incomplete quantifiers). It only passes in CI by replaying its RECORDED
0.3.6 hint. Under 0.3.7 the proof-lib VC shifts → the 0.3.6 hint no longer replays,
AND from-scratch still doesn't close → it fails. No fresh hint can be recorded
(a hint is only captured from a proof that closes, and it doesn't). Same shape for
`Simd128.squeeze2`.

### What was tried on `lemma_sq_lane_avx2_eq_squeeze_state` (ALL FAIL)
The 0.3.6 hint's `unsat_core` shows the proof uses a small fact set + ran at
fuel 1-2 / ifuel 0 (module default is fuel 0/ifuel 1 — it relied on F*'s auto
fuel-escalation), and does NOT touch `Core_models.Num` (rotate_left) or
`Hacspec_sha3.Keccak_f`. Levers tried:
- `--using_facts_from '* -Core_models.Num -Hacspec_sha3.Keccak_f'` (prune the
  changed-rotate_left / keccak cascade) — still incomplete quantifiers.
- `--fuel 2 --ifuel 0` (match the recorded config; ifuel 0 shrinks the search) —
  still fails.
- Factored the inline byte_eq into a STANDALONE `lemma_sq_lane_byte_eq_avx2`
  (mirrors the WORKING load-side `lemma_load_block_byte_eq_avx2`), clean top-level
  context — still fails.
- All three combined — still fails (494/800 used, ~18 min Z3 churn, incomplete
  quantifiers at fuel 2/ifuel 0).

- **`--using_facts_from '* -Rust_primitives.Slice.array_from_fn
  -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'`**
  (mirrors the exclusion the sibling AVX2 store proofs at `Simd.Avx2.Store.fst`
  lines 1293/1577 already use) — **big improvement but still fails**: 46 s / 360-800
  rlimit (vs 18 min / 494 before), still "incomplete quantifiers". So `array_from_fn`'s
  refinement-`forall` IS a real cascade contributor here (excluding it cut cost ~23×),
  but a SECOND layered cascade remains (skill §1.5.1). This exclusion is a KEEPER and
  should stay in the fix.

Conclusion: option tuning alone cannot close it (array_from_fn exclusion gets closest).
The residual is a second layered cascade — next step: smtprofile the residual failing
query (`--log_queries --z3refresh smt.qi.profile=true`) to find the dominant quantifier,
then either exclude/opacify it or spell out the per-byte store-ensures instantiation.
The per-byte equality
`sq_lane_avx2 ...[i] == squeeze_state ...[i]` needs the proof BODY spelled out:
explicitly instantiate `f_squeeze4`'s ensures (the `Libcrux_sha3.Simd.Avx2.Store`
impl per-byte `get_lane_u64` spec) at the right (lane l, word (i-start)/8, byte
(i-start)%8) for index i, and unfold `Hacspec_sha3.Sponge.squeeze_state` explicitly,
so Z3 doesn't have to search for the instantiation. This is the "incomplete
quantifiers = missing trigger, spell it out" case (skill §"Per-i match" / per-lane
seeds). Current state in this worktree: `lemma_sq_lane_byte_eq_avx2` exists
(factored) with `--fuel 2 --ifuel 0 --z3rlimit 800 --using_facts_from '...'`; body
still the original seed pattern — the BODY is what needs the explicit instantiations.

## How to iterate (fast)
Reuse the 154 0.3.7 `.checked` in `/tmp/sha3-037-cache` via fstar.exe direct
(content-hash dep reuse; make's mtime check would force a full rebuild after
re-extraction):
```
cd <worktree>/crates/algorithms/sha3/proofs/fstar/extraction
PREFIX=$(cat /tmp/wt_prefix_037.txt)   # worktree paths + d8b5b3d includes + /tmp cache
$PREFIX ../equivalence/EquivImplSpec.Sponge.Avx2.fst   # ~few min, deps cached
```
Judge by EXIT 0 + "All verification conditions discharged" (cache writes blocked by
corrupt repo `Core_models.Num.fst.checked`, but /tmp cache is writable).

## Next steps
1. Admit-walk inside `lemma_sq_lane_byte_eq_avx2` (drop `admit ()` after each step)
   to localize which instantiation is missing under 0.3.7.
2. Spell out the byte equality with explicit asserts (f_squeeze4/store ensures
   instantiated per index + squeeze_state unfolded). This should close it from
   scratch AND de-fragilize it at 0.3.6 (no longer hint-dependent).
3. Apply the same to `Simd128.squeeze2` (arm64) — likely the same class of fix.
4. Then full-tree gate under 0.3.7 (`make -k -j2 all CACHE_DIR=/tmp/sha3-037-cache/...`).

================================================================
## UPDATE 2026-05-31 (session 2) — deeper diagnosis, NOT yet closed clean
================================================================

### Corrected blocker locations
The full-tree gate (`make -k -j2 ... CACHE_DIR=/tmp/sha3-037-cache/...`) shows the
two ROOT failures are:
1. **`EquivImplSpec.Sponge.Avx2.lemma_sq_lane_byte_eq_avx2`** (avx2 squeeze byte-eq).
2. **`EquivImplSpec.Sponge.Arm64.Driver.lemma_squeeze2_arm64`** (NOT `Simd128.squeeze2` —
   that passes; the doc's "Simd128.squeeze2" really meant this driver lemma, which
   `Simd128.squeeze2` only *consumes* via the cached `.checked`).
`Generic_keccak.Simd128.squeeze2` itself verifies hint-free (12.8 s).

### Root-cause (confirmed)
- avx2 `Simd.Avx2.Store.store_block` packages its per-byte output spec in the OPAQUE
  `stored` predicate (`[@@ "opaque_to_smt"]`); arm64 `Simd.Arm64.Store.store_block`
  states the same `forall` DIRECTLY. That is why arm64 squeeze passes hint-free but
  avx2 needs the recorded (now-non-replaying) hint: Z3 can't instantiate a `forall` it
  can't see. Load side solves the twin via `reveal_opaque load_lane_u64`.
- Both lemmas select a lane by a symbolic `l` (`if l = 0 then .. else ..`), which forces
  a case split that fails hint-free under 0.3.7.

### What was built (verifies, KEEP)
- `lemma_stored_index` (consumer of opaque `stored`) and `lemma_squeeze_state_index`
  (consumer of `squeeze_state`'s post `forall`): each isolates ONE instantiation in a
  clean context where the `out.[k]` / `result.[i]` trigger fires trivially. Both verify
  in ~5 rlimit.
- `lemma_squeeze2_arm64`: rewritten with `match l` (concrete lanes) + a length-equality
  assert (`v outlen == Seq.length out0`) so the `squeeze2`-post guard discharges by
  substitution. `lemma_sq_lane_byte_eq_avx2`: rewritten with the two helpers + per-lane
  `match l` + the heavy `sq_lane == store_block-lane` reduction hoisted to a SINGLE
  `assert (lhs == sb_l)`.

### Why it is NOT closed: `--admit_except` masks a full-module ground explosion
Both lemmas VERIFY under `--admit_except <name>` (isolated: byte-eq 0.2 rlimit, squeeze2
214 rlimit) but FAIL in a clean full-module build at the rlimit cap (800):
- avx2: the single `assert (lhs == sb_l)` (f_squeeze4 #impl → store_block dictionary
  projection) now passes at 411 rlimit / 62 s, but the per-lane `prove` BODIES
  (queries 70/72) still time out at rlimit 800.
- arm64: one sub-VC (`r_l == squeeze ..`) times out at rlimit 800 / 150 s.
`smt.qi.profile` on the residual: peak quantifier-instance count ≈ 178 → NOT a
quantifier cascade. It is a ground-level term explosion that only appears when sibling
definitions / module SMTPats are active (i.e. clean), and is hidden by `--admit_except`.
This is the "passes --admit_except, fails clean / equality-beats-strict-ineq" trap
(libcrux memory `feedback_equality_beats_strict_ineq_composer`,
`feedback_opaque_predicate_store_proof`: "always finish with a full module build").

### Recommended next steps (for whoever resumes)
1. `smt.qi.profile` the EXACT failing query (avx2 query 70/72; arm64 query 9/12) — not
   `-4` — to see whether a specific SMTPat (e.g. `lemma_avx2_lane_eq_get_lane_u64`'s
   `[SMTPat (avx2_lane vec l)]`, or `createi_lemma`) or a heavy ground term
   (`squeeze` / `extract_lane` / the `store_block` dictionary body) dominates.
2. Likely fix: confine each per-lane proof to its OWN top-level lemma with a tight
   `--using_facts_from` that excludes the polluting SMTPats (call the lane bridge
   `lemma_avx2_lane_eq_get_lane_u64` EXPLICITLY instead of via its SMTPat), OR make the
   heavy reduced function opaque-to-smt at the call so its body never expands.
3. The current edits are a CORRECT skeleton (helpers verify; reduction isolates) — finish
   the full-module hardening, do NOT start from scratch.

### Iteration hygiene (learned the hard way this session)
- `--admit_except X` WRITES an admit-tainted `X.fst.checked`; a later no-admit run then
  *loads* it and reports a FALSE "All verification conditions discharged" in <1 s with NO
  `Query-stats` line. ALWAYS `rm /tmp/sha3-037-cache/checked/<Module>.fst.checked` before
  a "clean" judgement, and require a real `Query-stats ... succeeded` line (with a rlimit
  figure) — a sub-second pass with no query line is a cache load, not a proof.
- Judge ONLY by a clean, no-`--admit_except`, full-module run.

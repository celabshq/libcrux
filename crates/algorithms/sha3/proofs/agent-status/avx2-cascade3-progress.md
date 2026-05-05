# AVX2 cascade3 progress log

Mission: close third quantifier-cascade source in
`Libcrux_sha3.Simd.Avx2.load_block` (forall-25 cliff). Worktree
`/Users/karthik/libcrux-sha3-avx2`, branch `avx2-cascade`.

## 2026-05-05, T+0 (start)

- Worktree clean, branch `avx2-cascade` confirmed off
  `sha3-byteform-migration`.
- Verified prereq commits inherited:
  - `7bb581f8b` – `[@@ "opaque_to_smt"]` on `createi`
    (visible in `specs/sha3/proofs/fstar/extraction/Hacspec_sha3.fst`).
  - `8203c9ace` – `get_lane_u64_post` SMTPat lemma
    (verified diff in `crates/utils/intrinsics/src/avx2_extract.rs`;
     the F* extraction will be regenerated below since the
     `crates/utils/intrinsics/proofs/fstar/extraction/` directory has
     not yet been populated in this worktree).
- Next sub-task: enable `--log_queries --z3refresh --query_stats`
  on the load_block hax options in
  `crates/algorithms/sha3/src/simd/avx2.rs`, run
  `bash crates/algorithms/sha3/hax.sh extract` to populate the F*
  extraction directories, then run `make check/...Avx2.fst` with
  `--admit_except` set on `load_block` to capture the failing
  `.smt2`.
- Blocker: none yet.
- ETA: ~10 min for extraction; ~5 min for first failing query
  identification.

## 2026-05-05, T+15 (after profiling)

- Extraction succeeded (`/tmp/avx2-extract.log` exit 0). Intrinsics
  F* in `crates/utils/intrinsics/proofs/fstar/extraction/` regenerated;
  `get_lane_u64_post` SMTPat lemma confirmed in extracted .fsti.
- `make check/Libcrux_sha3.Simd.Avx2.fst OTHERFLAGS="--admit_except
  Libcrux_sha3.Simd.Avx2.load_block"` reproduces the 4 failing
  sub-queries (qs 692-695 at line 1091 first per-iteration assert,
  q 796 at line 1164 second assert). Each cancels at 400/400 in
  ~80-100s.
- z3 qi.profile of `queries-Libcrux_sha3.Simd.Avx2-692.smt2` (1.7MB,
  the i=0 entry into the unrolled 4-lane assertion):
  - `k!61` 1,096,670 instantiations (max gen 11).
  - `refinement_interpretation_Tm_refine_cda1...` 560k (Slice fsti
    line 20:65 — array_from_fn index refinement).
  - `Tm_refine_8143...` 493k (Slice fsti line 20:7 — array_from_fn
    body).
  - `lemma_get_lane_u64_post` only 159k — both inherited fixes ARE
    biting (createi opaque, get_lane_u64 SMTPat).
- Z3 `trace=true` decoded `k!61` to the F* prelude axiom
  `;;fuel irrelevance` (Pulse/F* SMT prelude, declared right under
  `(declare-fun HasTypeFuel ...)`):
  ```
  (forall ((f Fuel) (x Term) (t Term))
   (! (= (HasTypeFuel (SFuel f) x t) (HasTypeZ x t))
    :pattern ((HasTypeFuel (SFuel f) x t))))
  ```
  No `:qid`, single pattern. Fires on every `HasTypeFuel (SFuel _)`
  in the goal — and the iterated `array_from_fn` refinement-interp
  axioms are the producer.
- This is upstream of hax-lib (it lives in the F* SMT prelude).
  The fix is to suppress the array_from_fn-tagged refinements that
  funnel `HasTypeFuel (SFuel _)` terms into k!61. Trying option (c)
  from brief: per-fn `--using_facts_from '* -Rust_primitives.Slice.array_from_fn'`.
- Next sub-task: enable that filter, drop the debug flags, retry.
- Blocker: none.
- ETA: ~5 min retry.

## 2026-05-05, T+30 (filter results — early)

- Re-extracted with
  `--using_facts_from '* -Rust_primitives.Slice.array_from_fn'`
  on the load_block hax options. Debug flags removed.
- `make check/Libcrux_sha3.Simd.Avx2.fst OTHERFLAGS="--admit_except
  load_block"` past sub-query 692 (the previously-failing first
  iteration assert) — succeeded at ~80ms, used_rlimit 0.56. Same
  for 690 / 691.
- Still running for the remaining ~200+ sub-queries; will report
  final EXIT and any remaining failures.

## 2026-05-05, T+0 (phase3-start)

- Picking up Phase 3 from commit `28db4222a` on `avx2-cascade`.
- Sub-task: implement Step 1 of next-attempt path — opacify
  `load_lane_u64` via `[@@ "opaque_to_smt"]` AND add an SMTPat
  extensionality lemma `load_lane_u64_lane_extensionality`.
  Inject via `hax_lib::fstar::after` in `crates/algorithms/sha3/src/simd/avx2.rs`
  (NOT in extracted F*).
- Plan: pattern after `get_lane_u64_post` SMTPat in
  `crates/utils/intrinsics/src/avx2_extract.rs`:55-87. Lemma signature
  taken from extracted `Libcrux_sha3.Simd.Avx2.fst:72-96`.
- Blocker: none.
- ETA: ~15 min for edit + extract + first make.

## 2026-05-05, T+phase3-result (continued by parent after agent stall)

The phase-3 agent stalled inside watchdog after editing simd/avx2.rs
(opacify + SMTPat lemma + reveal_opaque calls in load_u64x4x4 and
load_u64x4 bodies). Parent picked up the make-check.

### Setup

- Re-extraction succeeded (exit 0, ~5 min). Extracted Avx2.fst now
  has `[@@ "opaque_to_smt"]` markers at lines 70 (load_lane_u64),
  191 (load_u64x4x4), 420 (load_u64x4), and the
  `load_lane_u64_lane_extensionality` lemma at L124 (val) / L156 (let).

### Verification result — partial improvement

- `make check/Libcrux_sha3.Simd.Avx2.fst`: SIGTERM at 20-min timeout.
- `load_lane_u64_lane_extensionality` itself verified with hint in
  79-100ms (rlimit 80, used 0.13-0.17). load_u64x4x4 (~325 sub-queries)
  and load_u64x4 verified clean.
- load_block: 1005 sub-queries total. 992 fast (<1s), 8 slow but
  passed (max 939ms / 15s for one outlier). 5 failed at cliff
  (~95s used_rlimit 400):
  - sub-query 799 → assert at line 1233 (k=1: state[4*i+1])
  - sub-queries 902, 903, 904 → assert at line 1318 (k=2: state[4*i+2])
  - sub-query 1005 → assert at line 1403 (k=3: state[4*i+3])
- ✅ k=0 assertion at line 1160 (state[4*i]) — all sub-queries pass
  fast. The Phase-3 lemma DOES bridge the load_lane_u64-level
  equality from the loop_invariant for the first per-iteration
  assertion.
- ❌ k=1, k=2, k=3 assertions at lines 1233, 1318, 1403 — same shape
  as k=0, identical lemma applies in principle, but the SMTPat does
  not fire / does not close.

### Diagnosis (open question)

All four asserts have identical shape:
```fstar
get_lane_u64 (state.[4*i + k]) lane ==
load_lane_u64 blocks offset (4*i + k) (old_state.[4*i + k]) lane
```
After 4 set_ij calls (one per k), state[4*i+k] = g_k where g_k is
the k-th component of load_u64x4x4's tuple output. The flat-index
projection `(4*i+k)/5 * 5 + (4*i+k)%5 == 4*i+k` is the same identity
for all k.

Hypotheses for asymmetric closure (NOT YET tested):

1. `set_ij`'s post-condition projection lemma may have an SMTPat
   that fires more eagerly for the most-recent set or for the
   first-set element, depending on Z3's pattern-instantiation order.
2. load_u64x4x4's post is a tuple-projected ensures; the per-component
   accessor `(g0, g1, g2, g3)` may decompose only g0 cleanly (because
   F* tuples often have stronger SMTPats on the .fst component).
3. The using_facts_from filter might happen to suppress some axiom
   that's needed for k≥1 but whose presence was the original cliff
   for k=0.

### Next-attempt path (to attempt next)

- **First**: tighten the SMTPat or add a second pattern. Try
  `[SMTPat (load_lane_u64 blocks offset i s1 lane); SMTPat (load_lane_u64 blocks offset i s2 lane)]`
  (both load_lane_u64 calls trigger). Or supplement with
  per-k explicit `hax_lib::fstar!(r#"load_lane_u64_lane_extensionality $blocks $offset (mk_usize 4 *! $i +! mk_usize $k) ..."#)`
  hint calls before the failing asserts.
- **Second**: attempt explicit Seq.upd projection lemma about
  `set_ij` chain — assert `state[4*i+k] == g_k` separately via
  `hax_lib::fstar!` before each lane assert. This is the bridge
  the SMTPat cannot synthesize.
- **Third**: split each per-iteration assert into 4 separate
  load_lane_u64-level asserts (one per lane) — narrower SMT goals
  may close where the conjoined goal does not.
- **Last resort**: factor each of the 4 asserts into a separate
  Lemma with `--split_queries always --z3rlimit 800` and explicit
  precondition listing `get_lane_u64 (state.[4*i+k]) lane ==
  get_lane_u64 g_k lane`.

### Files to commit on `avx2-cascade`

- crates/algorithms/sha3/src/simd/avx2.rs (Phase-3 source edits;
  diff +86 lines).
- crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.fst
  (regenerated extraction).
- crates/algorithms/sha3/proofs/agent-status/avx2-cascade3-progress.md
  (this file).

## 2026-05-05, T+phase3-close — CLEAN (re-extracted from Rust source)

### Root cause of the partial close

qi.profile of the failing sub-queries (q693 line 1160 / k=0, q797 line
1233 / k=1) revealed: **the SMTPat-tagged lemma added in Phase 3 had
itself become the new dominant cascade**, firing 3.95M / 4.58M times
(7× more than k!61). The original multi-pattern was asymmetric in
trigger specificity:
```
[SMTPat (load_lane_u64 blocks offset i s1 lane);
 SMTPat (Libcrux_intrinsics.Avx2_extract.get_lane_u64 s2 lane)]
```
The second trigger `get_lane_u64 s2 lane` matches every `get_lane_u64`
in scope (loop_invariant has 4*i × 4 = up-to-100 such calls). Combined
with the load_lane_u64 trigger, the cross-product saturated Z3.

The k=0 / k≠0 asymmetry was an artifact: with the broad pattern, Z3 got
*lucky* on whichever sub-query happened to find a useful instantiation
first. With `--z3refresh` (each query starts fresh), even k=0 failed —
a clue the pattern, not the chain depth, was the issue.

### Fix landed

Tighten the second trigger to share more universal vars with the first:
```
[SMTPat (load_lane_u64 blocks offset i s1 lane);
 SMTPat (load_lane_u64 blocks offset i s2 lane)]
```
This restricts firing to pairs of `load_lane_u64` calls with matching
`(blocks, offset, i, lane)` differing only in state arg — exactly the
assert-vs-load_u64x4x4-post bridge case. Edit in
`crates/algorithms/sha3/src/simd/avx2.rs` lines 103-104 and 138-139
(both `val` and `let` SMTPat blocks).

### Verification

- `make check/Libcrux_sha3.Simd.Avx2.fst` — clean. 1361 load_block
  sub-queries, max 1.5 s (was 95 s cancelling at rlimit 400).
- 0 errors / 0 cancellations / `.checked` written.
- `cd ../equivalence && make` — fails on missing
  `EquivImplSpec.Sponge.SqueezeFrame.fst` which is the parallel
  squeeze2 agent's domain (file not yet on `sha3-byteform-migration`),
  per brief §"file boundaries". NOT an Avx2 regression.

### Method

- Original failure surface (5 sub-queries, lines 1233/1318/1318/1318/1403)
  → re-ran make with `--log_queries --query_stats --z3refresh` on
  load_block options to capture .smt2 files.
- z3 4.13.3 with `smt.qi.profile=true` on the failing .smt2 → top
  quantifier was the lemma I had added.
- Tightened the SMTPat → re-ran make with `--admit_except load_block`
  → clean (1076 queries, max 35 s).
- Removed debug flags, ported edit to Rust source, re-extracted via
  `bash crates/algorithms/sha3/hax.sh extract`, ran full make → clean.

The cascade-fix-introducing-new-cascade pattern is documented in
`fstar-for-libcrux` skill §1.5.1.

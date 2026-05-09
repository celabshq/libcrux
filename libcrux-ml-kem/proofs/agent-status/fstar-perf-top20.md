# F* verification time — top-20 culprits

Standing instruction (per memory `feedback_track_fstar_perf`): refresh
this file after every full F* build (`python3 hax.py prove` or full
`make`).  Append new snapshots; keep prior ones for regression tracking.

Use awk/python to parse `Query-stats` lines from the build log:
- per-function `total = Σ ms` and `max single query ms`
- count of queries
- `failed` flag (Z3 returned unknown / canceled)
- `saturated` flag (`used rlimit > 0.8 * rlimit`)

---

## Snapshot 1 — 2026-04-28 — `verification_result.txt` from prior session (tip `ba8681b38`-ish)

Source: `libcrux-ml-kem/verification_result.txt` (last `hax.py prove`
exit 0 from morning track-A session).  All ml-kem-related modules.

| # | Total (s) | Max query (ms) | Queries | Failed | rlimit-sat | Module | Function |
|---|---|---|---|---|---|---|---|
| 1 | 31.9 | 31739 | 3 | 1 | 1 | Libcrux_ml_kem.Invert_ntt | invert_ntt_at_layer_4_plus |
| 2 | 30.4 | 30437 | 1 | 0 | 0 | Hacspec_ml_kem.Commute.Chunk | lemma_base_case_mult_even_mod_core |
| 3 | 24.7 |  1796 | 225 | 42 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_4_plus |
| 4 |  8.5 |   461 | 75 | 0 | 0 | Libcrux_ml_kem.Vector.Avx2 | op_ntt_layer_3_step |
| 5 |  8.2 |   150 | 75 | 2 | 0 | Libcrux_ml_kem.Vector.Avx2 | op_inv_ntt_layer_3_step |
| 6 |  7.2 |  3644 | 2 | 0 | 0 | Libcrux_ml_kem.Polynomial | v_ZETAS_TIMES_MONTGOMERY_R |
| 7 |  7.0 |   121 | 79 | 17 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_7_ |
| 8 |  7.0 |   100 | 81 | 6 | 0 | Libcrux_ml_kem.Polynomial | add_message_error_reduce |
| 9 |  6.7 |   212 | 70 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_inv_ntt_layer_3_step |
| 10 |  6.5 |   121 | 70 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_ntt_layer_3_step |
| 11 |  5.6 |   164 | 57 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_inv_ntt_layer_2_step |
| 12 |  5.3 |   113 | 60 | 40 | 0 | Libcrux_ml_kem.Polynomial | add_to_ring_element |
| 13 |  5.3 |   116 | 57 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_ntt_layer_2_step |
| 14 |  5.1 |   121 | 57 | 6 | 0 | Libcrux_ml_kem.Polynomial | subtract_reduce |
| 15 |  5.0 |   153 | 54 | 0 | 0 | Libcrux_ml_kem.Ntt | ntt_vector_u |
| 16 |  4.8 |   129 | 56 | 16 | 0 | Libcrux_ml_kem.Polynomial | ntt_multiply |
| 17 |  4.3 |   101 | 52 | 5 | 0 | Libcrux_ml_kem.Polynomial | add_error_reduce |
| 18 |  4.2 |    97 | 50 | 6 | 0 | Libcrux_ml_kem.Polynomial | add_standard_error_reduce |
| 19 |  3.4 |  3302 | 2 | 0 | 0 | Hacspec_ml_kem.Commute.Chunk | lemma_ntt_layer_1_step_lane_bridge |
| 20 |  3.2 |   127 | 33 | 1 | 0 | Libcrux_ml_kem.Vector.Portable | op_decompress_1_ |

### Observations

- **Two outliers dominate**: `invert_ntt_at_layer_4_plus` (#1) and
  `lemma_base_case_mult_even_mod_core` (#2) each cost ~30 s on a single
  query — i.e., one heavy SMT problem each, not many small ones.
  **#1's Q2 hit the rlimit ceiling (saturated) — known borderline.**
  Both are candidates for proof restructuring or temporary admits.
- **`ntt_at_layer_4_plus` (#3)**: 225 queries with 42 failures.
  High failure rate suggests intermittent flakiness or unhinted paths.
- **The 7 `Polynomial` reduce-family fns** (#6, #8, #12, #14, #16,
  #17, #18) collectively cost ~38 s.  Most are 50-80 queries each at
  100-130 ms max — these are the strengthened Phase 7a fns from
  agents E2 and (in flight) trackD.
- **The 6 `op_{ntt,inv_ntt}_layer_{2,3}_step` wrappers** in
  Vector.Portable + Vector.Avx2 cost ~36 s combined — Phase 7b agent
  F's forward layer 1 path is reflected here.
- **`Hacspec_ml_kem.Commute.Chunk`** has just 1 outlier each
  (#2 even base-case, #19 ntt-layer-1 lane bridge) — the Tier-2
  algebra dominates over everything else in that file.

### Bypass-during-experiment proposals

| Function | Today's strategy | Bypass during edit-iter |
|---|---|---|
| `invert_ntt_at_layer_4_plus` (#1) | rlimit 200 | wrap in `--admit_smt_queries true` while iterating on `_1`/`_2`/`_3` (saves ~32 s + retry costs per Invert_ntt build) |
| `ntt_at_layer_4_plus` (#3) | hint replay (some failures) | same admit pattern when iterating elsewhere in Ntt.fst |
| `lemma_base_case_mult_even_mod_core` (#2) | calc-style + rlimit 400 | leave as-is — once-and-done query |
| Polynomial reduce fns (#8, #12, #14, etc.) | strengthened by E2 / trackD | edit one at a time, admit the others |

---

## Snapshot 2 — 2026-04-28 afternoon — `Invert_ntt.fst.checked` build only (post-Step-4, tip `8358b1093`)

Source: `/tmp/inv_ntt_optB.log`.  Single-module rebuild after lane (a)
Step 4 strengthening landed.

| # | Total (s) | Max query (ms) | Queries | Failed | Function |
|---|---|---|---|---|---|
| 1 | 269.6 | 68995 | 464 | 0 | invert_ntt_at_layer_1_ |
| 2 | 222.7 | 28218 | 195 | 1 | invert_ntt_at_layer_4_plus |
| 3 | 2.1 | 1964 | 2 | 0 | invert_ntt_at_layer_2_ |
| 4 | 0.7 | 579 | 2 | 0 | invert_ntt_at_layer_3_ |
| 5 | 0.2 | 141 | 2 | 0 | invert_ntt_montgomery |

Total wall time: 528 s (8.8 min).

### Observations

- **`invert_ntt_at_layer_1_` blew up to 270 s** because of the
  strengthened post (rlimit 800, `--split_queries always`, 464 split
  queries).  Expected; this is the active edit.
- **`invert_ntt_at_layer_4_plus` is HALF the cost (223 s)** even when
  not edited.  Q2 fails at rlimit 200 ("canceled"), F* retries
  successfully.  This is the dominant baseline overhead in any
  Invert_ntt rebuild.  Hints don't help its borderline queries.
- **`_2`, `_3`, `_montgomery` replay in 3 s combined** — hint replay
  works fine for them.
- **Iteration speedup target**: bypassing `_4_plus` saves ~3.7 min
  per Invert_ntt rebuild.  For active work on layers 1/2/3 only, an
  `--admit_smt_queries true` wrap is a major savings.

### Open question

- Why does `invert_ntt_at_layer_4_plus` Q2 always fail under hint
  replay?  Either the hint is stale (proof structure changed since
  the hint was recorded) or the borderline rlimit doesn't tolerate
  hint cost.  Worth investigating once Phase 7a is closed —
  re-recording hints might recover the savings.

---

## Reducing F* iter-loop turnaround — proven techniques

Three orthogonal mechanisms.  Use them in this order of preference
(fastest first):

### α. fstar-mcp `typecheck_buffer` — sub-second feedback

Skip `make` entirely during inner-loop iteration on a single .fst file.
The fstar-mcp server (port 3001) holds a long-running F* session with
the file's deps already loaded; `typecheck_buffer` re-verifies the
buffer in sub-second wall time.  See memory `feedback_use_fstar_mcp.md`
and skill at `~/.claude/skills/fstar-mcp/`.

**When to use**: editing F* lemma bodies in `Hacspec_ml_kem.Commute.Chunk`
or `Hacspec_ml_kem.Commute.Bridges`; iterating on the body proof of a
single `.fst` after re-extracting once.

**When NOT to use**: after `python3 hax.py extract`, the .fst is
regenerated from Rust source — the fstar-mcp session's view of the .fst
becomes stale.  Need to `update_buffer` or re-create session.  For
edit-Rust-then-verify cycles, either still use fstar-mcp (refresh
buffer after each extract) or fall back to `make` (slower but clean).

### β. Per-function admit during iteration — `#[hax_lib::fstar::options("--admit_smt_queries true")]`

Apply on Rust-side functions you're NOT actively editing.  This injects
`#push-options "--admit_smt_queries true"` around that function's body
in the extracted .fst, so its proof obligations pass trivially.

**Saves ~222s per Invert_ntt rebuild** if applied to
`invert_ntt_at_layer_4_plus` while iterating on `_1`/`_2`/`_3` (per
Snapshot 2 above: 4_plus is half the wall cost when hint replay fails
on its borderline Q2).

**Pattern** (do NOT commit; revert before end-of-step verify):
```rust
#[hax_lib::fstar::options("--admit_smt_queries true")]  // TEMP — see Step 2 iter notes
pub(crate) fn invert_ntt_at_layer_4_plus<...>(...) { ... }
```

**Workflow**:
1. Add the attribute to the irrelevant fn(s) in `src/invert_ntt.rs`.
2. `python3 hax.py extract` (re-renders .fst).
3. Iterate via `make`/fstar-mcp on your target fn.
4. **Before commit**: remove the attribute, re-extract, run a clean
   `make` to confirm regression-clean.

### γ. Whole-module admit via `ADMIT_MODULES` Makefile var

Already used for `Libcrux_ml_kem.Ind_cpa.fst`, `Libcrux_ml_kem.Ind_cca.Unpacked.fst`,
and all `Vector.Neon.*` modules (per `Makefile:5-13`).

**When to consider adding more**: end-of-step regression checks where
a heavy module dirty-rebuilds despite not being on the active proof
path.  Likely candidates if needed:

- `Libcrux_ml_kem.Sampling.fst` (~1056 stats-lines in last full prove,
  one of the heaviest).  Not on the INTT critical path.
- `Libcrux_ml_kem.Serialize.fst` (~246 stats-lines).  Not on INTT path.
- `Libcrux_ml_kem.Ntt.fst` (forward NTT, ~630 stats-lines).  Not on
  the inverse-NTT path.

**Don't admit** on the active critical path (`Polynomial.fst`,
`Invert_ntt.fst`, `Vector.{Avx2,Portable}.fst`,
`Hacspec_ml_kem.Commute.{Chunk,Bridges}.fst`, `Vector.Traits.fst`).
Admit-during-debugging hides regressions in those.

### Recommended discipline for the INTT critical path (Steps 2-5)

| Phase | Best tool | Bypass |
|---|---|---|
| Step 2 layer 3 / 2 (Bridges.fst lemma writing) | fstar-mcp typecheck_buffer | n/a — Bridges.fst doesn't need Invert_ntt rebuilt |
| Step 3 layer 4_plus bridge (Bridges.fst) | fstar-mcp | n/a |
| Step 4 layer 2/3/4_plus Rust strengthening | `make Invert_ntt.fst.checked` | per-fn admit on the OTHER layers + `_montgomery` |
| Step 5 `invert_ntt_montgomery` post chain | `make` (no shortcuts; needs all layers) | none |
| End of each step | full `make` regression | remove all temp admits |

---

## Snapshot 2 — 2026-04-29 — Wave-B baseline (above-trait worktree, `fa31480cd`)

**Source:** `/tmp/wave-b-baseline-take3.log` (full make from
`~/libcrux-ml-kem-above-trait/libcrux-ml-kem/proofs/fstar/extraction/`).
**Wall:** ~9 min cold.  **Errors:** 0 (108 hint-replay warnings, all
F* auto-retried successfully — F* IDE sessions on Bridges.fst /
Ind_cpa.fst kept warm in source worktree concurrently).

### NOT directly comparable to Snapshot 1

Wave-B's local Makefile admits the entire below-trait surface
(`Vector.{Portable,Avx2}.*`) plus Wave-C's consumer chain (Matrix,
Ind_cca.*, Mlkem*).  Plus a TEMP admit on `Libcrux_ml_kem.Invert_ntt.fst`
because its `inv_ntt_layer_int_vec_step_reduce` Q101 saturates at
rlimit 200 in the without-hint retry path (hint-replay fails first,
then no-hint retry hits 200/200 used in 57 s).  Lane A5 will
UNADMIT Invert_ntt.fst when it begins (A5 owns this module per
wave-B-prompt §"WAVE-B SCOPE").

### Wave-B verification surface (top entries above 0.05 s)

| # | Total (s) | Max query (ms) | Queries | Failed | rlimit-sat | Module | Function |
|---|---|---|---|---|---|---|---|
| 1 | 4.9 | 4867 |  1 | 0 | 0 | Libcrux_ml_kem.Serialize | compress_then_serialize_message |
| 2 | 1.2 |  142 | 42 | 0 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_4_plus |
| 3 | 1.2 | 1179 |  1 | 0 | 0 | Libcrux_ml_kem.Serialize | deserialize_to_reduced_ring_element |
| 4 | 1.0 |  966 |  1 | 0 | 0 | Libcrux_ml_kem.Serialize | deserialize_then_decompress_message |
| 5 | 0.5 |   50 | 16 | 0 | 0 | Libcrux_ml_kem.Polynomial | ntt_multiply |
| 6 | 0.4 |   28 | 17 | 0 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_7_ |
| 7 | 0.3 |   64 | 11 | 0 | 0 | Libcrux_ml_kem.Polynomial | add_to_ring_element |
| 8 | 0.2 |  145 |  2 | 0 | 0 | Libcrux_ml_kem.Polynomial | multiply_by_constant_bounded |
| 9 | 0.1 |   28 |  6 | 0 | 0 | Libcrux_ml_kem.Polynomial | add_message_error_reduce |
| 10 | 0.1 |   25 |  6 | 0 | 0 | Libcrux_ml_kem.Polynomial | add_standard_error_reduce |
| 11 | 0.1 |   26 |  5 | 0 | 0 | Libcrux_ml_kem.Polynomial | add_error_reduce |

### Observations

- **`compress_then_serialize_message` 4.9 s, 1 query, max 4867 ms** —
  a single heavy Z3 problem.  Above-trait `Serialize.fst`; A1's Phase 7c
  migration touches this module.  Watch for regression when A1 adds
  hacspec citations.
- **`ntt_at_layer_4_plus` 1.2 s / 42 queries** vs Snapshot 1's 24.7 s /
  225 queries.  Hint replay is doing most of the work — most queries
  succeed immediately on hint match.  A5's strengthening work could
  invalidate these hints; A5 should expect rlimit-sat regressions
  when it touches Chunk.fst / Bridges.fst.
- **A3 targets all under 0.4 s** in the warm-cache state:
  `add_to_ring_element` 0.3 s, `add_message_error_reduce` 0.1 s,
  `add_standard_error_reduce` 0.1 s, `add_error_reduce` 0.1 s.
  USER-7's `subtract_reduce` is admitted via `--admit_smt_queries true`
  on the body (so no Query-stats reach the table).  When A3 unadmits
  the body, expect these 4 functions' Z3 cost to jump.
- **Hacspec_ml_kem.Commute.Chunk lemmas not in the table** — those live
  in `specs/ml-kem/proofs/fstar/commute/` (separate sub-Makefile), so
  Query-stats for them appear only when `make` is run from THAT dir.
  Wave-B's per-lane work needs to refresh that dir's perf data
  separately (or run with `--query_stats` from a top-level prove).

### Regression-watch thresholds for Wave-B

- **A1**: alert if `compress_then_serialize_message` total grows
  >7.4 s (1.5×) or its max query >7300 ms.  Other Serialize fns
  similarly.
- **A2**: Sampling.fst not currently in top-11 (warm-cache + lax
  bypasses); A2's `lax→panic_free` removal will introduce new
  Query-stats lines.  Establish per-fn baseline at A2 start.
- **A3**: alert if `add_*_reduce` family totals jump >0.5 s or any
  of them shows >5 saturated rlimit queries.  USER-7's
  `subtract_reduce` will appear in the table for the first time
  when A3 unadmits.
- **A5**: A5 unadmits Invert_ntt.fst.  Expect `inv_ntt_layer_int_vec_step_reduce`
  Q101 saturation to recur (this is the Step 5 spike target).
  Also watch `invert_ntt_at_layer_4_plus` (Snapshot 1: 31.9 s / 1
  failed query at rlimit 200) — currently bypassed via
  `--admit_smt_queries true` per `agent-trackA.md` "Layer 4_plus
  regression — diagnosis + landing decision".

---

## Snapshot 3 — 2026-04-29 — Wave-B 4-lane parallel merge (`tip TBD post-push`)

**Source:** `/tmp/wave-b-merged-baseline-take5.log` (full make from
`~/libcrux-ml-kem-above-trait/libcrux-ml-kem/proofs/fstar/extraction/`
post-merge of all 4 Wave-B lane branches + Makefile cleanup +
Phase 6d Neon `.fsti` admit + duplicate-`noeq` workaround on
regenerated Vector.Neon.Vector_type.fsti).
**Wall:** ~10 min cold.  **Errors:** 0.

### Wave-B parallel-fanout outcome

Lanes closed: A1 (-1, `to_unsigned_field_modulus`), A3 (-1 via
`subtract_reduce` body discharge with USER-7 array-form bridge fix
+ unshadowing trick).  Filed: A2 (USER-10 with smtprofiling-grade
4-path diagnostic), A5 (Steps 3.3/4/5 strengthened with USER-13/14/15
on bodies; layer_2 newly admitted as USER-13).  **Net admit-count
delta: -2 PROGRESS, +3 USER-N filings, 0 SIDEWAYS.**

### Top-20 culprits (post-merge)

| # | Total (s) | Max query (ms) | Queries | Failed | rlimit-sat | Module | Function |
|---|---|---|---|---|---|---|---|
| 1 | **153.6** | **153301** | 26 | 1 | 1 | Hacspec_ml_kem.Commute.Bridges | lemma_inv_ntt_layer_2_step_lane_bridge |
| 2 | 22.1 | 22139 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Portable.Arithmetic | to_unsigned_representative |
| 3 |  8.8 |  2592 | 110 |  0 |  0 | Libcrux_ml_kem.Polynomial | subtract_reduce |
| 4 |  8.7 |  2714 |  22 |  0 |  0 | Libcrux_ml_kem.Vector.Portable.Compress | decompress_1_ |
| 5 |  4.8 |  4804 |   1 |  0 |  0 | Libcrux_ml_kem.Serialize | compress_then_serialize_message |
| 6 |  4.0 |  4028 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | op_serialize_4_post_bridge |
| 7 |  3.9 |   142 | 121 |  0 |  0 | Libcrux_ml_kem.Invert_ntt | invert_ntt_at_layer_1_ |
| 8 |  3.0 |  3037 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | impl_3 |
| 9 |  2.2 |   736 |  43 |  0 |  0 | Libcrux_ml_kem.Invert_ntt | inv_ntt_layer_int_vec_step_reduce |
| 10 | 1.5 | 1546 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Portable | impl_1 |
| 11 | 1.4 | 1383 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Portable.Arithmetic | multiply_by_constant |
| 12 | 1.2 |  145 |  42 |  0 |  0 | Libcrux_ml_kem.Ntt | ntt_at_layer_4_plus |
| 13 | 1.2 | 1174 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | op_serialize_4_pre_bridge |
| 14 | 1.1 | 1099 |   1 |  0 |  0 | Libcrux_ml_kem.Serialize | deserialize_to_reduced_ring_element |
| 15 | 0.9 |   33 |  38 |  0 |  0 | Libcrux_ml_kem.Invert_ntt | invert_ntt_at_layer_3_ |
| 16 | 0.9 |  897 |   2 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | op_serialize_11_post_bridge |
| 17 | 0.9 |  876 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Portable.Compress | compress_message_coefficient |
| 18 | 0.9 |  875 |   1 |  0 |  0 | Libcrux_ml_kem.Serialize | deserialize_then_decompress_message |
| 19 | 0.6 |  645 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | op_deserialize_4_post_bridge |
| 20 | 0.6 |  622 |   1 |  0 |  0 | Libcrux_ml_kem.Vector.Avx2 | op_serialize_10_post_bridge |

### Major regression — needs investigation

**`lemma_inv_ntt_layer_2_step_lane_bridge` jumped 3.4 s → 153.6 s**
(Snapshot 1 → Snapshot 3, 45×).  1 query failed (presumably hint
replay miss → without-hint retry succeeded after 153 s); 1 query
saturated rlimit.  Likely cause: A5's new helpers in `Chunk.fst`
(Phase 7a / lane A5 additions section + parameter unshadowing) plus
A3's array-form lemmas changed the dependency graph hashes, so the
old `Bridges.fst` hint cache no longer replays.

**Recommendation for next session:** re-record hints for Bridges.fst
(`make check/Hacspec_ml_kem.Commute.Bridges.fst` once with
`--record_hints`, then commit hints).  This should drop the 153 s
back to ~3-5 s.

**Update 2026-04-29 15:55 (post-merge):** hint re-record was attempted.
Outcome was partial:

| Lemma | Pre-record | Post-record | Notes |
|---|---|---|---|
| `lemma_ntt_layer_1_step_lane_bridge` | 35.6 s (Snap 1) | 224.7 s recording / TBD replay | 2 queries, 0 fail — hint replay should drop it |
| `lemma_inv_ntt_layer_1_step_lane_bridge` | 57.9 s (history) | 29.3 s | 2 queries, hint-replay clean |
| `lemma_inv_ntt_layer_2_step_lane_bridge` | 153.6 s (Snap 3) | **146.2 s** | 27 queries, **1 fail + 1 rlimit-sat** — hint replay does NOT fix |
| `lemma_inv_ntt_layer_3_step_lane_bridge` | 43.4 s (history) | 31.5 s | 2 queries, hint-replay clean |

The layer_2 lemma's slowness is NOT a hint-cache problem — Z3 has a
persistent quantifier wall.  This matches A5's filing of layer_2 as
USER-13.  The proper fix is the SD3 opaque-wrapper pattern (per
`feedback_layer2_branch_post_z3_unlock`): wrap the per-chunk post in
`[@@ "opaque_to_smt"]` and use 4 per-branch helpers + per-lane wrapper
+ `--split_queries always`.  This is the same pattern that closed the
forward layer_2 in commit `b7b49c358`.

### Other notable changes vs Snapshot 1

- `to_unsigned_representative` 8.5 s → 22.1 s (2.6×) — same hint
  cache invalidation pattern; A1's removal of `panic_free` from its
  caller `to_unsigned_field_modulus` means the caller now propagates
  the trait post into Serialize, which routes through this fn.
- `subtract_reduce` 5.1 s → 8.8 s — A3's body now actually verifies
  (was admitted in Snapshot 1).  Net benefit: -1 admit; 1.7× cost is
  acceptable.
- `compress_then_serialize_message` 4.9 s → 4.8 s — flat, A1's
  closure didn't perturb this consumer.
- `Vector.Avx2.op_serialize_*_bridge` family appears for the first
  time at 0.5-4.0 s — these are the bridges B3 closed in the
  parallel below-trait session at `e5c4a6f49`.  Genuine new
  verification work, not regressions.

### Wave-B local Makefile workaround (2026-04-29)

Wave-B's local Makefile in `~/libcrux-ml-kem-above-trait/` was
accidentally committed by lane A5 (`aae3046a9`) and reverted by the
coordinator (`7e75d3d7c`).  The local Makefile remains in the
worktree (uncommitted) for future sessions to reuse.  Phase 6d
`.fsti` admit (`22b5c016e`) is upstream-clean and committed.

### hax codegen bug — duplicate `noeq` on Vector_type.fsti

`hax extract` regenerates `Libcrux_ml_kem.Vector.Neon.Vector_type.fsti`
with TWO `noeq` qualifiers on `t_SIMD128Vector` (Error 162 at line
10-13 — "Duplicate qualifiers").  Workaround applied LOCALLY in
above-trait's gitignored .fsti via `sed`.  Source worktree avoids
this because its `src/vector/neon/vector_type.rs` has uncommitted
edits that produce a different .fsti shape.  **File as USER-N:**
hax codegen bug; track separately from the F* verification work.

---

## Snapshot 4 — 2026-04-29 evening — USER-13 closed (lane A5, post-rebase)

Source: lane `agent/lane-A5` rebased onto `trait-opacify`
(`8bf91ca56`) + USER-13 fix.  Full-tree make EXIT=0 in `~659 s`
(replay; Bridges.fst and Invert_ntt cached during full-tree run).
Per-module cold rebuilds for the affected modules:

  * `Libcrux_ml_kem.Invert_ntt.fst`: cold record-hints, 126.4 s.

### USER-13 closure — `invert_ntt_at_layer_2_`

| Metric | Snapshot 3 | Snapshot 4 |
|---|---|---|
| Function pragma | `--admit_smt_queries true` | `--z3rlimit 100` |
| Total wall | n/a (admit) | **2.1 s** |
| Max single query | n/a | 2.0 s |
| Queries | 0 | 2 |
| Failed | 0 | 0 |
| rlimit-saturated | 0 | 0 |

Closure mechanism: the prior body emitted a global
`reveal_opaque (\`%is_i16b_array_opaque) (is_i16b_array_opaque)` that
unfolded the opaque bound predicate into a 16-lane forall — pollution
that the iter-end loop-invariant subtyping check (Q108) ran into,
saturating rlimit 200/400/800 from 42 s through hangs past 12 min.
Replacing the global reveal with a single targeted post-call assert
of the per-iteration chunk bound (`assert (is_bounded_vector 6656
re.coefficients.[round])`) gives Z3 the bound atom directly, avoiding
the `forall4 inv_ntt_layer_2_step_branch_post` FE-algebra conjunct
that the trait post still carries alongside the bound.  No new opaque
wrapper or Bridges/Chunk lemma was needed — the proven `b7b49c358`
machinery (4 per-branch + per-lane wrapper + per-vector bridge) is
not on this code path; the wall was purely in the impl-side body.

### Layer_2 lemma in Bridges.fst — perf carry-over

`lemma_inv_ntt_layer_2_step_lane_bridge` is unchanged
(`Hacspec_ml_kem.Commute.Bridges.fst` not edited).  Per Snapshot 3
this lemma was **146.2 s post-hint-record**.  It remains the top-1
single-fn perf consumer in the full tree — recommend a separate
follow-up (USER-N: persistent quantifier wall, not USER-13 path).

### Invert_ntt.fst per-fn (cold rebuild, 2026-04-29 18:43)

| # | Total (s) | Max (ms) | Queries | Function |
|---|---|---|---|---|
| 1 | 41.1 | 138 | 585 | `invert_ntt_at_layer_1_` |
| 2 | 22.9 | 572 | 298 | `invert_ntt_at_layer_3_` |
| 3 | 10.2 | 114 | 121 | `inv_ntt_layer_int_vec_step_reduce` |
| 4 | **2.1** | 2028 | 2 | **`invert_ntt_at_layer_2_` (USER-13 closed)** |
| n/a | (admitted) | — | — | `invert_ntt_at_layer_4_plus` (USER-14 carry-over) |
| n/a | (admitted) | — | — | `invert_ntt_montgomery` (USER-15 carry-over) |

Note: `invert_ntt_at_layer_1_` 41 s vs Snapshot 3's 3.9 s is a
hint-replay vs cold-record artifact; run was without prior hint
cache hits (will replay cleanly on next warm rebuild).

### Top-20 (full-tree, partial — Bridges/Chunk/Invert_ntt cached)

| # | Total (s) | Max (ms) | Queries | Function |
|---|---|---|---|---|
| 1 | 24.7 | 20718 | 3 | `Vector.Portable.Arithmetic.to_unsigned_representative` |
| 2 | 23.9 | 1490 | 267 | `Ntt.ntt_at_layer_4_plus` |
| 3 | 10.8 | 2719 | 49 | `Vector.Portable.Compress.decompress_1_` |
| 4 |  9.7 |  128 | 104 | `Vector.Avx2.Serialize.serialize_4_` |
| 5 |  8.6 | 8503 |   2 | `Vector.Portable.Arithmetic.montgomery_reduce_element` |
| 6 |  7.8 |  443 |  75 | `Vector.Avx2.op_ntt_layer_3_step` |
| 7 |  7.8 |  103 | 102 | `Polynomial.add_to_ring_element` |
| 8 |  7.7 |  127 |  80 | `Vector.Avx2.Serialize.serialize_10___serialize_10_vec` |
| 9 |  7.7 |  121 |  80 | `Vector.Avx2.Serialize.serialize_12___serialize_12_vec` |
| 10 | 7.6 |  128 |  77 | `Vector.Avx2.op_inv_ntt_layer_3_step` |
| 11 | 7.6 | 1977 | 110 | `Polynomial.subtract_reduce` |
| 12 | 6.9 |  109 |  96 | `Ntt.ntt_at_layer_7_` |
| 13 | 6.6 |   94 |  86 | `Polynomial.add_message_error_reduce` |
| 14 | 6.4 | 2630 |  47 | `Vector.Avx2.Ntt.ntt_layer_3_step` |
| 15 | 6.2 |  279 |  67 | `Vector.Avx2.Serialize.serialize_10_` |
| 16 | 6.1 |  191 |  70 | `Vector.Portable.op_inv_ntt_layer_3_step` |
| 17 | 6.1 |  111 |  70 | `Vector.Portable.op_ntt_layer_3_step` |
| 18 | 6.0 |  159 |  67 | `Vector.Avx2.Serialize.serialize_12_` |
| 19 | 5.9 |  158 |  81 | `Vector.Portable.Arithmetic.get_n_least_significant_bits` |
| 20 | 5.1 |  131 |  71 | `Polynomial.ntt_multiply` |

(`Bridges.lemma_inv_ntt_layer_2_step_lane_bridge` would still place
top-1 at 146 s if Bridges.fst weren't .checked-cached during the
full make.)

### Summary delta vs Snapshot 3

  * **USER-13 admit removed** — net `-1` admit, `-1` SIDEWAYS conversion.
  * `invert_ntt_at_layer_2_` body verifies in 2.1 s vs admitted before.
  * No regressions in the full-tree make (`EXIT=0`, all VC discharged).
  * `lemma_inv_ntt_layer_2_step_lane_bridge` still at 146 s (no change,
    no fix attempted by this lane — separate follow-up needed).
  * Critical-path forward: USER-15 (`invert_ntt_montgomery`) and USER-14
    (`invert_ntt_at_layer_4_plus` body) remain the next bodies to
    discharge.

### USER-13 lesson — Rule SD4 added to lane-split-protocol

The closure mechanism is now codified as **Rule SD4** in
`lane-split-protocol.md` and as a top-of-page banner in
`handoff-2026-04-29-end-of-day.md`.  TL;DR: never use the GLOBAL
`reveal_opaque (\`%P) (P)` form inside a loop body — it unfolds the
opaque predicate universally and pollutes Z3 every iteration with the
underlying forall.  Prefer a targeted `assert (P specific-args)`
first; if you need a reveal, use `reveal_opaque (\`%P) (P arg1 arg2)`
with full arguments.

### Reveal-opaque audit (2026-04-29 evening, post-USER-13 closure)

Audit ran across all `*.rs` (inside `hax_lib::fstar!` blocks) and
`proofs/fstar/extraction/*.fst[i]` looking for the GLOBAL form
`reveal_opaque (\`%X.foo) (X.foo)` (no instance arguments) inside loop
bodies or hot subtyping checks.  Findings:

| Risk | File | Line | Form | Context | Status |
|---|---|---|---|---|---|
| ~~HIGH~~ ✅ **CLOSED** | `src/invert_ntt.rs` | 268–269 | GLOBAL | `invert_ntt_at_layer_3` body, inside `for round in 0..16` | **Closed in commit `200b01f66`.**  Removed global reveal; added two targeted asserts (bound + spec-function equation).  Pragma `--z3rlimit 800 --ext context_pruning --split_queries always` → `--z3rlimit 200` (8% Z3 budget used, 2.1 s warm wall, 2 queries). |
| MEDIUM | `src/invert_ntt.rs` | 79 | TARGETED | `invert_ntt_at_layer_1` body, inside `for round in 0..16` | Already targeted (full args).  Low risk; leave alone unless that fn becomes a perf hot-spot. |
| MEDIUM | `src/serialize.rs` | 153 | TARGETED | `deserialize_12_short`, inside `cloop chunks` | Already targeted (`is_i16b_array_opaque 4095`).  Low risk; leave alone. |
| LOW | `src/polynomial.rs` | 59, 71, 86, 101, 120, 133 | GLOBAL | Top-level helper functions (`is_bounded_vector_higher`, `add_bounded`, `sub_bounded`, etc.) — no loop | Once-per-call.  Structural; correct. |
| LOW | `src/ind_cpa.rs` | 908, 1170 | GLOBAL | Top-level entry of `encrypt`/`decrypt`, no loop | Once-per-call.  Structural; correct. |
| LOW | `Hacspec_ml_kem.Commute.{Bridges,Chunk}.fst` | various | GLOBAL | Inside lemma proofs that DEFINE the unfolded form | By design.  Don't change. |

**SD4 status: lane is now SD4-clean.**  No remaining GLOBAL `reveal_opaque (\`%P) (P)` form inside any loop body in `invert_ntt.rs` or other impl-side `*.rs`.  The MEDIUM cases use the targeted form correctly.  Future audits should track new occurrences during code review using the regex `reveal_opaque\s*\(\`%[\w.]+\)\s*\(\s*[\w.]+\s*\)` (no instance arguments) inside `hax_lib::fstar!` blocks.
## Snapshot 2 — 2026-05-08 — cold-baseline at tip `c07306a5b` (post-deserialize_5 + audit)

Source: `/tmp/cold_prove.log` from `make -k -j2 -C proofs/fstar/extraction/`
after deleting all 177 ml-kem-related `.checked` files.  Quiet machine
(load avg 3.4, 0 other fstar.exe at start).  Total Z3 wall time across
all queries: **~22 min**.  Total modules attempted: 1525.

`make rc=2` due to **2 cold-baseline failures** (latent — not addressed
in this snapshot, see §Failures below).

### Top 25 by total per-function time

| # | Total (s) | Max query (ms) | N | Failed | rlimit-sat | Module | Function |
|---|---|---|---|---|---|---|---|
| 1 | 86.1 | 70891 | 57 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_ntt_layer_2_step |
| 2 | 83.9 | 71457 | 57 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_inv_ntt_layer_2_step |
| 3 | 75.6 | 73583 | 70 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_inv_ntt_layer_3_step |
| 4 | 55.3 | 51127 | 70 | 0 | 0 | Libcrux_ml_kem.Vector.Portable | op_ntt_layer_3_step |
| 5 | 48.8 |   516 | 586 | 121 | 0 | Libcrux_ml_kem.Invert_ntt | invert_ntt_at_layer_1_ |
| 6 | 46.7 |   201 | 553 | 92 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_1_ |
| 7 | 43.7 |  7553 | 285 | 116 | 0 | Libcrux_ml_kem.Ind_cca.Unpacked | decapsulate |
| 8 | 35.4 |   288 | 397 | 54 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_2_ |
| 9 | 32.6 | 32600 | 1 | 0 | 0 | Hacspec_ml_kem.Commute.Chunk | lemma_base_case_mult_even_mod_core |
| 10 | 28.2 |  1811 | 267 | 42 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_4_plus |
| 11 | 28.1 |  9200 | 362 | 1 | 1 | Libcrux_ml_kem.Vector.Avx2 | impl_3 |
| 12 | 25.9 |   146 | 298 | 44 | 0 | Libcrux_ml_kem.Ntt | ntt_at_layer_3_ |
| 13 | 19.5 |   551 | 219 | 32 | 0 | Libcrux_ml_kem.Vector.Portable.Ntt | ntt_multiply |
| 14 | 11.9 |   157 | 104 | 5 | 0 | Libcrux_ml_kem.Vector.Avx2.Serialize | serialize_4_ |
| 15 | 11.9 |   144 | 122 | 6 | 0 | Libcrux_ml_kem.Invert_ntt | inv_ntt_layer_int_vec_step_reduce |
| 16 | 11.7 |   140 | 142 | 9 | 0 | Libcrux_ml_kem.Vector.Portable.Ntt | ntt_multiply_binomials |
| 17 | 11.5 |   221 | 90 | 4 | 0 | Libcrux_ml_kem.Vector.Avx2.Serialize | serialize_1_ |
| 18 | 10.8 |   153 | 91 | 0 | 0 | Libcrux_ml_kem.Vector.Avx2.Serialize | serialize_5___serialize_5_vec |
| 19 | 10.7 |   151 | 109 | 1 | 0 | Libcrux_ml_kem.Ind_cpa | encrypt_c1 |
| 20 | 10.7 |   302 | 109 | 0 | 0 | Libcrux_ml_kem.Ind_cpa | encrypt_unpacked |
| 21 | 9.5 |   116 | 119 | 8 | 0 | Libcrux_ml_kem.Polynomial | subtract_reduce |
| 22 | 9.2 |   143 | 78 | 2 | 0 | Libcrux_ml_kem.Vector.Avx2 | op_inv_ntt_layer_3_step |
| 23 | 9.2 |   143 | 80 | 0 | 0 | Libcrux_ml_kem.Vector.Avx2.Serialize | serialize_12___serialize_12_vec |
| 24 | 9.2 |  8996 | 3 | 0 | 0 | Libcrux_ml_kem.Vector.Portable.Arithmetic | montgomery_reduce_element |
| 25 | 9.1 |   149 | 80 | 0 | 0 | Libcrux_ml_kem.Vector.Avx2.Serialize | serialize_10___serialize_10_vec |

### Notable observations

1. **Items 1–4 (Portable op_(inv_)ntt_layer_{2,3}_step) dominate**: each
   has ONE 51–73 s query that takes most of its total. These are the
   functions we just bumped to `rlimit 600/800 --fuel 1 --split_queries
   always` in commits `7a206f303` and `f335c2a87`. The bump worked but
   these proofs sit at the rlimit edge — the single-query max is 50–70 s
   wall on a quiet machine. **High contention risk** (any parallel SHA-3
   / ML-DSA work running can push these over).

2. **Item 9: `lemma_base_case_mult_even_mod_core` (32.6 s, 1 query)** —
   single Montgomery base-case lemma in `Hacspec_ml_kem.Commute.Chunk`.
   This is a **prime qi.profile candidate** — single 32 s query is a clean
   target for cascade-source identification. Likely cascading on the
   `mod_q_eq` body's raw `% 3329` arithmetic, which is exactly what the
   audit flagged as L7/L8 leakage. Closing L7+L8 should close or
   significantly reduce this one.

3. **Items 5, 6, 8, 12 (NTT/inverse-NTT layer_N_):** these are the
   NTT-driving sprint's targets. High query count + medium-low max-query
   means they're heavily split. Failed counts (54–121) are mostly "with
   hint" retries that succeed (per `feedback_no_cache_nuke`).

4. **Item 7: `Ind_cca.Unpacked.decapsulate` (43.7 s, max 7.5 s)** —
   end-to-end IND-CCA composition. Heavy on integration over the trait
   posts. Sensitive to trait-post sharpness.

5. **Item 24: `Vector.Portable.Arithmetic.montgomery_reduce_element`
   (9.2 s, 3 queries, max 9.0 s)** — the Montgomery reduce primitive
   itself. Sub-query 1 takes ~9 s. **Direct relevance to L8** — this
   is what `montgomery_multiply_by_constant_post` cites.

6. **Item 11: `Vector.Avx2.impl_3` (28.1 s, max 9.2 s, 362 queries, 1
   saturated rlimit)** — typeclass instance for AVX2's `t_Operations`.
   The 1 saturated rlimit is a query at the edge.

### Failures (cold-baseline, latent)

Two `Error 19` failures that the parent worktree's hot cache hid:

1. **`Libcrux_ml_kem.Types.Index_impls.fst:18`** — Subtyping check on
   `Core_models.Ops.Index.f_index_pre self.f_value i`. rlimit=15
   (default). "incomplete quantifiers". The `self.f_value.[ index ]`
   call's pre is not deriving. Likely needs a small rlimit/fuel bump in
   the source `types.rs` or per-impl pre strengthening.

2. **`Libcrux_ml_kem.Vector.Portable.fst:1008`** — Could not prove
   post-condition for `f_from_bytes` at rlimit=80 fuel=0. This is the
   Sprint B trait wrapper we just discharged via `panic_free` + strong
   ensures (commit `49e70d5d4`). The post fails at the impl
   `f_from_bytes_post` field check — likely the trait subtype check is
   re-doing the proof at lower rlimit than the body did. Either:
   - bump the file-level rlimit in `vector/portable.rs::f_from_bytes`
     impl block, OR
   - add a `reveal_opaque` for the relevant lane atoms.

Both are minor; flag for a quick follow-up sprint.

### Implications for next-sprint planning

| Concern | Cold-baseline signal | Next-sprint impact |
|---|---|---|
| Items 1–4 (Portable ntt-layer-step bumped at 600/800) | Single-query 50–73 s on a quiet machine | NTT-driving consumers will inherit this latency. Consider rerunning qi.profile on op_ntt_layer_2_step's slowest sub-query to find structural fix. |
| Item 9 (`lemma_base_case_mult_even_mod_core` 32 s in 1 query) | Single heavy query in Montgomery base case | qi.profile this — likely directly addressed by L8 (Montgomery post opacification). |
| Item 24 (`montgomery_reduce_element` 9 s in 3 queries) | Sub-query #1 dominates | L8 cleanup should reduce this |
| 2 cold-baseline failures (Index_impls, Portable.f_from_bytes) | rlimit 15/80 — both at file default | Quick fix sprint (~30 min) — bump rlimit on the impl trait wrappers |

**Recommendation for the L7+L8 sprint (~5 hours per audit estimate):**
- Run qi.profile on item 9 (`lemma_base_case_mult_even_mod_core`) FIRST
  to confirm the cascade source matches the audit's L7+L8 prediction.
- If profile confirms: do L8 (Montgomery), re-snapshot, expect items 9 +
  24 to drop. Then L7 (Barrett).
- If profile reveals a different cascade: revise the cleanup plan.

## Snapshot 2 — 2026-05-08 — `impl_3` split_queries test (Track B validation)

**Source:** `/private/tmp/.../tasks/buohvkj2y.output` (worktree
`libcrux-impl3-split-test`, parent tip `4118f74a4`).

**Change tested:** added `#[cfg_attr(hax, hax_lib::fstar::options("--split_queries always"))]`
on the `impl Operations for SIMD256Vector` block in
`libcrux-ml-kem/src/vector/avx2.rs::1165`. No other edits.

**Result:**
| Metric | Value |
|---|---|
| extract_rc | 0 |
| make_rc | 0 |
| total queries | 363 |
| failed | 0 |
| saturated | 0 |
| top-3 query ms | 202, 197, 174 |
| total ms | 18 260 (≈18 s) |
| max query ms | 202 |

Every query stays well under the 400 ms-with-`split_queries` rlimit
budget. Track B (`--split_queries always` on impl block) is validated
as a general fix for one-line-dispatcher impl records.

**Application plan** (deferred until L7+L8 agent's branch merges, since
both files are touched by the agent):
- `vector/avx2.rs::1165` `impl Operations for SIMD256Vector` — perf
  cleanup; impl_3 currently a top-25 cold-time contributor.
- `vector/portable.rs::950` `impl Operations for PortableVector` — also
  expected to fix the `Vector.Portable.fst:1008` cold-baseline failure
  (the `f_from_bytes` post check at the impl-record level is a
  combined-WP issue that splits cleanly per-method).

## Snapshot 2 — 2026-05-09 — post-L7+L8 cold-baseline at tip `9dc1fa2cd`

**Source:** `/tmp/parent-fullmake.log` (full `make -k -j4` from
`libcrux-ml-kem/proofs/fstar/extraction/`, wall 6:13).

**Build status:** `rc=2` due to **1 cold-baseline failure**:
`Libcrux_ml_kem.Types.Index_impls.fst:18` — pre-existing, unrelated
to L7+L8 (Task #22; needs spec strengthening of `f_index_pre`).

**Wall time:** 6:13 (down from 13:53 in agent's worktree's first try
without copied hints — the hint regeneration sprint here in the
parent was the main effect).

### Top 20 by single-query max time

| max ms | total ms | function |
|---:|---:|---|
| 35 220 | 59 140 | `Libcrux_ml_kem.Vector.Avx2.Serialize.deserialize_5_` |
| 9 392 | 9 481 | `Libcrux_ml_kem.Vector.Portable.Arithmetic.montgomery_reduce_element` |
| 6 709 | 6 838 | `Libcrux_ml_kem.Vector.Portable.Arithmetic.to_unsigned_representative` |
| 5 705 | 5 835 | `Libcrux_ml_kem.Vector.Avx2.Arithmetic.to_unsigned_representative` |
| 5 660 | 5 891 | `Libcrux_ml_kem.Invert_ntt.invert_ntt_at_layer_3_` |
| 4 875 | 5 176 | `Libcrux_ml_kem.Invert_ntt.invert_ntt_at_layer_2_` |
| 3 895 | 7 586 | `Libcrux_ml_kem.Polynomial.v_ZETAS_TIMES_MONTGOMERY_R` |
| 3 724 | 3 829 | `Libcrux_ml_kem.Vector.Portable.Compress.compress_1_` |
| 3 515 | 4 199 | `Libcrux_ml_kem.Serialize.deserialize_then_decompress_ring_element_u` |
| 3 489 | 7 947 | `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_layer_3_step` |
| 2 437 | 2 531 | `Libcrux_ml_kem.Vector.Portable.Compress.decompress_ciphertext_coefficient` |
| 2 396 | 2 511 | `Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_3_step` |
| 1 979 | 4 635 | `Libcrux_ml_kem.Polynomial.multiply_by_constant_bounded` |
| 1 836 | 1 954 | `Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_2_step` |
| 1 758 | 1 889 | `Libcrux_ml_kem.Vector.Portable.Sampling.rej_sample` |
| 1 722 | 26 464 | `Libcrux_ml_kem.Ntt.ntt_at_layer_4_plus` |
| 1 642 | 1 770 | `Libcrux_ml_kem.Vector.Portable.Ntt.ntt_layer_1_step` |
| 1 623 | 1 763 | `Libcrux_ml_kem.Vector.Portable.Ntt.inv_ntt_layer_1_step` |
| 1 324 | 1 586 | `Libcrux_ml_kem.Ind_cpa.serialize_vector` |
| 1 248 | 1 597 | `Libcrux_ml_kem.Serialize.deserialize_to_reduced_ring_element` |

### Top 20 by total per-function

| total ms | max ms | function |
|---:|---:|---|
| 59 140 | 35 220 | `Libcrux_ml_kem.Vector.Avx2.Serialize.deserialize_5_` |
| 48 984 | 567 | `Libcrux_ml_kem.Invert_ntt.invert_ntt_at_layer_1_` |
| 46 841 | 204 | `Libcrux_ml_kem.Ntt.ntt_at_layer_1_` |
| 34 838 | 291 | `Libcrux_ml_kem.Ntt.ntt_at_layer_2_` |
| 26 464 | 1 722 | `Libcrux_ml_kem.Ntt.ntt_at_layer_4_plus` |
| 25 957 | 163 | `Libcrux_ml_kem.Ntt.ntt_at_layer_3_` |
| 21 195 | 645 | `Libcrux_ml_kem.Vector.Portable.Ntt.ntt_multiply` |
| 17 807 | 259 | `Libcrux_ml_kem.Ind_cca.Unpacked.decapsulate` |
| 12 628 | 175 | `Libcrux_ml_kem.Vector.Portable.Ntt.ntt_multiply_binomials` |
| 12 195 | 170 | `Libcrux_ml_kem.Vector.Avx2.Serialize.deserialize_5___deserialize_5_vec` |
| 11 936 | 178 | `Libcrux_ml_kem.Invert_ntt.inv_ntt_layer_int_vec_step_reduce` |
| 10 983 | 188 | `Libcrux_ml_kem.Vector.Avx2.Serialize.serialize_4_` |
| 10 876 | 219 | `Libcrux_ml_kem.Vector.Avx2.Serialize.serialize_1_` |
| 10 595 | 142 | `Libcrux_ml_kem.Ind_cpa.encrypt_c1` |
| 10 567 | 277 | `Libcrux_ml_kem.Ind_cpa.encrypt_unpacked` |
| 10 348 | 201 | `Libcrux_ml_kem.Vector.Avx2.Serialize.serialize_5___serialize_5_vec` |
| 10 309 | 138 | `Libcrux_ml_kem.Polynomial.subtract_reduce` |
|  9 481 | 9 392 | `Libcrux_ml_kem.Vector.Portable.Arithmetic.montgomery_reduce_element` |
|  8 830 | 194 | `Libcrux_ml_kem.Ind_cca.Unpacked.generate_keypair` |
|  8 748 | 128 | `Libcrux_ml_kem.Polynomial.add_to_ring_element` |

### Comparison vs Snapshot 2 (2026-05-08, pre-L7+L8)

The audit predicted L7+L8 would drop items 9 (`lemma_base_case_mult_even_mod_core`,
32 s in one query) and 24 (`montgomery_reduce_element`, 9 s in 3 queries).

- `montgomery_reduce_element`: 9 392 ms in **1** query (was 9 s in 3
  queries) — **same wall**, just consolidated. The non-linear `* 169`
  arithmetic that the L8 abstraction hides is consumed *inside* this
  function, not above the trait, so per-function cost is unchanged.
- `lemma_base_case_mult_even_mod_core`: not in current top-20.
  Successfully verified within `Hacspec_ml_kem.Commute.Chunk` —
  expected since the lemma's body now consumes the new opaque
  `montgomery_multiply_lane_post` atom that wraps the residue, and the
  `mod_q_eq_unfold` happens inside `Commute.Chunk` lemma bodies (the
  L7+L8 reveal discipline) rather than at every consumer.

**Net effect of L7+L8 on the top-25:** small to negligible at the
*single-query max* level — the non-linear arithmetic was already
locked inside opaque atoms (`mod_q_eq`) at parent.  The win is
*architectural*: above-trait callers (`Polynomial.fst`, `Ind_cpa.fst`,
`Invert_ntt.fst`) no longer see raw `% 3329` or `* v c * 169` in their
goals, removing accidental-NRA risk for future code added to those
files.

### New cliff-edge candidates worth qi.profile

- `deserialize_5_` (35 s single query) — by far the heaviest, dominates
  cold time.  Pre-existing; not L7+L8-related.
- `to_unsigned_representative` (Portable+Avx2): both ~6 s single query.

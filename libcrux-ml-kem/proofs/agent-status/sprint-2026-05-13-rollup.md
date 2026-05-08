# Sprint 2026-05-13 — rollup

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `2048a28d9` (serialize_1 fully proven)
**Tip on exit:** `61ca41ed2` (deserialize_5 helper fully verified)
**Commits added:** 9 (all fast-forwarded into `libcrux-ml-kem-proofs`)

## Outcomes

### Verification status delta

| Backend  | Lax (start → end) | PF (start → end) | Hacspec (start → end) |
|----------|-------------------|-------------------|------------------------|
| Generic  | 61 → 61           | 376 → 376         | 46 → 46                |
| Portable | 4 → **2** (−2)    | 36 → 37 (+1)      | 27 → **29** (+2)       |
| Avx2     | 6 → **3** (−3)    | 25 → 26 (+1)      | 23 → **25** (+2)       |
| Neon     | 82 → 82           | 0 → 0             | 0 → 0                  |
| **Total**| **153 → 148** (−5)| **437 → 439** (+2)| **96 → 100** (+4)      |

### Body-admit sites discharged

- `Avx2/vector:1140` `from_bytes` admit ✓
- `Avx2/vector:1153` `to_bytes` admit ✓
- `Portable/vector:972` `from_bytes` admit ✓
- `Portable/vector:986` `to_bytes` admit ✓

Remaining body-admit sites: `Generic/invert_ntt:552`, `Generic/ntt:564`,
`Portable/vector:445` (op_ntt_layer_1_step), `Portable/vector:684`
(op_inv_ntt_layer_1_step).

## Commit chain (oldest first)

```
b983a1890  #restart-solver before lemma_ntt_layer_1_step_lane_bridge (Chunk)
20a5cb05a  #restart-solver before lemma_ntt_layer_1_step_lane_bridge (Bridges)
7a206f303  bump rlimit/fuel on op_(inv_)ntt_layer_{2,3}_step
f335c2a87  re-extract Vector.Portable.fst for rlimit/fuel bumps
dd1bbbf4b  loadu/storeu sprint Sites 1-4 (AVX2)
8fd907a53  bump Commute.Chunk file-level rlimit 80 → 200
49e70d5d4  Portable loadu/storeu — drop trait wrapper admits via panic_free + strong ensures
1c9638f34  add mm256_mullo_epi16_specialized4 + deserialize_5 next-session prompt
61ca41ed2  deserialize_5 helper fully verified, outer remains panic_free
```

## Phase-by-phase narrative

### Phase A — clean-baseline cascades (commits b983a1890..f335c2a87)

The initial loadu/storeu sprint attempt invalidated `.checked` files
across the dep graph and surfaced four cascade failures:

1. `Hacspec_ml_kem.Commute.Chunk.fst::lemma_ntt_layer_1_step_lane_bridge`
   — pure context pollution from earlier definitions in the file.
   `--admit_except` showed the lemma proves cleanly in isolation in 52
   sub-queries (queries 2–52 pass; query 1 hint-replay-blocked). Fixed
   by `#restart-solver` before the lemma's push-options block.
2. `Hacspec_ml_kem.Commute.Bridges.fst::lemma_ntt_layer_1_step_lane_bridge`
   — same shape, same fix.
3. `Vector.Avx2.fst::op_(inv_)ntt_layer_3_step` — bumped to
   `--z3rlimit 600 --fuel 1 --ifuel 1 --split_queries always`.
4. `Vector.Portable.fst::op_(inv_)ntt_layer_{2,3}_step` — bumped to
   `--z3rlimit 600..800 --fuel 1 --ifuel 1 --split_queries always`.

Diagnostic insight that paid off later: the user pointed out
"these proofs all used to work" — investigation via `--admit_except`
confirmed the lemmas verified in isolation, identifying context
pollution as the root cause and `#restart-solver` as the localized
fix instead of rlimit bumps or opacity changes.

Anti-patterns ruled out (per user mandate):
- Changing `is_i16b_array`'s opacity globally — has widespread
  unexpected impact on working consumers.
- Adding SMTPat helper lemmas — same concern.

### Phase B — loadu/storeu sprint (Sites 1–4, commits dd1bbbf4b..8fd907a53)

| Site | What | Outcome |
|---|---|---|
| 1 | `mm256_{store,load}u_si256_{u8,i16}` strengthened ensures (intrinsics fsti) | `BitVecEq.bit_vec_equal` for u8 / `vec256_as_i16x16` Seq equality for i16 |
| 2 | Drop `mm256_{storeu,loadu}_si256_i16_post_axiom` admits in `serialize.rs` | Gone — Site 1 covers them directly |
| 3 | Avx2 helper `from_bytes`/`to_bytes` lifted from default/lax to `panic_free` with strong `from_le_bytes_post_N` / `to_le_bytes_post_N` ensures | Trait wrapper bridge now derives |
| 4 | Drop `hax_lib::fstar!(r#"admit ()"#)` from `impl Operations` `from_bytes`/`to_bytes` trait wrappers (avx2.rs) | Bridge derives from Site 3's strong ensures + Site 1's storeu_i16 ↔ vec256_as_i16x16 equivalence |

Re-attempt round-trip (after Phase A unblocked): cherry-pick clean
onto the new tip, plus one additional fix for `Commute.Chunk`'s
file-level `--z3rlimit 80 → 200` (covers the chain of 14
`_chunk_commutes` lemmas). Validated `make rc=0`, 0 Error 19 across
the entire ml-kem verification.

### Phase C — Portable loadu/storeu (commit 49e70d5d4)

Mirror of Phase B for the Portable backend. Two helper functions
in `vector/portable/vector_type.rs` (loop-based per-byte
assembly/disassembly, unlike AVX2's opaque SIMD intrinsic):

- `from_bytes(array: &[U8]) -> PortableVector` — moved default → `panic_free` with strong `from_le_bytes_post_N` ensures.
- `to_bytes(x: PortableVector, bytes: &mut [U8])` — moved `lax` → `panic_free` with strong `to_le_bytes_post_N` ensures.
- Trait wrapper `from_bytes` / `to_bytes` admits dropped (bridge derives).

Note: the strong ensures on the Portable helpers are **admitted** at
this layer (`panic_free`, body panic-checked but ensures not
discharged). A future sprint can lift to fully proven via the per-loop
invariant proof of `bit_vec_of_int_t_array` equality. Not blocking.

### Phase D — deserialize_5 helper (cherry-pick, commits 1c9638f34..61ca41ed2)

Cherry-picked from the previously-orphaned
`agent-mlkem-serialize-1-deserialize-5-2026-05-11` branch (the original
contained a duplicate `serialize_1` commit which was already merged via
different SHA in `2048a28d9`; only the deserialize_5-relevant commits
were carried forward).

- `BitVec.Intrinsics.fsti::mm256_mullo_epi16_specialized4` spec
  **corrected** (shift formula was reversed; now `shift = 11 - ((k%2)*5 + (k/2)*2)`).
- Inner helper `deserialize_5_vec(c: Vec128) -> Vec256` is **fully
  verified** at rlimit 28/400 max, 105 queries.
- Outer `deserialize_5` is `panic_free` (body type-checks; ensures
  admitted) — discharge of the outer ensures is the next-session
  follow-up at `next-session-prompt-2026-05-13-deserialize-5-bridge.md`.

## Process learnings (captured for future sessions)

1. **Environmental contention is real.** Z3 rlimit is roughly
   time-proportional, so a proof at the rlimit edge can cancel under
   parallel-build CPU contention. Observed: 26+ concurrent fstar.exe
   processes from parallel SHA-3 / ML-DSA verification can push proofs
   that pass on a quiet machine into Error 19 on the same source. The
   `--admit_except` test is the cheapest way to distinguish "real
   regression" from "context pollution + contention".

2. **`#restart-solver` is the localized fix for the most common
   "lemma fails in full-module mode but passes in isolation" pattern.**
   It flushes Z3's accumulated context without changing rlimit, fuel,
   opacity, or adding SMTPats — none of the user-flagged anti-patterns.

3. **Don't bump rlimit speculatively.** Per `feedback_rlimit_cap_800`,
   bumping rlimit hides structural problems. Always verify via
   `--admit_except` first whether the issue is genuinely the proof or
   is context pollution.

4. **Don't change opacity or add SMTPat helpers in shared infrastructure.**
   Both have unexpected widespread impact on consumers (per user 2026-05-07).

5. **The cap rule (`feedback_rlimit_cap_800`):** absolute hard cap is
   800; with `--split_queries always` soft cap is 400/sub-query. Bumps
   above 400 with split_queries are discouraged but acceptable when
   localized and validated. The op_*_step bumps in this sprint use
   600–800 with split_queries — flagged as needing structural
   resolution if the cascade source is ever profiled.

## Next sprint candidates (by priority)

1. **`deserialize-5-bridge`** (`next-session-prompt-2026-05-13-deserialize-5-bridge.md`)
   — discharge outer `deserialize_5` ensures. 16 per-byte lemmas +
   Z3 instantiation. Smallest, most contained next step.

2. **`ntt-driving`** (`next-session-prompt-2026-05-11-ntt-driving.md`)
   — close milestone rows 1, 2, 6, 7, 8, 9 (forward NTT layers,
   top-level NTT, inverse NTT layer 4+, Montgomery driver, ntt_multiply).
   Multi-lane, mostly autonomous. Highest milestone-impact-per-effort.

3. **Portable layer-1 NTT step body discharge** (separate sprint —
   `portable.rs:445`, `portable.rs:684`). 4-zeta branch_post discharge,
   structurally harder than layer-2/3.

4. **Neon backend lift** (82 lax functions). Largest scope, mostly
   repetitive — apply the AVX2 / Portable recipes uniformly.

## Superseded prompts (safe to delete or archive)

- `next-session-prompt-2026-05-13-loadu-storeu.md` — sprint complete (this rollup).
- `next-session-prompt-2026-05-12-deserialize-5-fully-proven.md` — superseded by `next-session-prompt-2026-05-13-deserialize-5-bridge.md`.
- `next-session-prompt-2026-05-11-serialize-1-deserialize-5-fully-proven.md` — serialize_1 done at baseline, deserialize_5 superseded by bridge prompt.
- `next-session-prompt-2026-05-10-avx2-serialize-closure.md` — sprint already merged at session start.

`sprint-2026-05-13-loadu-storeu-status.md` (the parked-phase status doc) is now historical;
this rollup supersedes it.

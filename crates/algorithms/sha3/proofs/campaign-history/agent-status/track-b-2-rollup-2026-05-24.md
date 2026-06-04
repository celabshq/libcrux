# Track B-2 rollup — 2026-05-24

Worktree: `/Users/karthik/libcrux-sha3-track-b-2`
Branch: `sha3-proofs-track-b-2`
Base HEAD: `d083da011`
Mission: close admits 4 and 5 (inner `byte_eq` `admit ();` in
`lemma_load_block_eq_xor_block_into_state_{arm64,avx2}`)

## Outcome

- **Admits closed**: 0 of 2.
- **Architecture improvement**: structural extraction in place
  (`lemma_load_block_byte_eq_{arm64,avx2}` top-level lemmas, outer
  lemmas reduced to thin wrappers).
- **Step 1 (opacity foundation) validated end-to-end**: `Keccakf.Arm64`
  closes 710 ms, `Keccakf.Avx2` closes 1.9 s on a warm cache (cold full
  build ~13 min through Avx2.Load deps).
- **Step 2 (companion squeeze lemma fix) — Arm64 only**: adding
  `KA.lemma_arm64_lane_unfold state.[j] l` in
  `lemma_sq_lane_arm64_eq_squeeze_state`'s `byte_eq` closes the
  Fix-1-induced regression. Arm64 squeeze passes 26 s wall at 161/800
  rlimit.
- **Step 2 — Avx2 fails**: same companion fix attempted on
  `lemma_sq_lane_avx2_eq_squeeze_state` insufficient. Sub-query 1
  cancels at 800/800 in 155 s; split-retry sub-query 102 cancels at
  178 s. The 4-lane variant has structurally larger cascade than 2-lane.
- **Step 3 (structural extraction) — body still admitted**:
  the standalone lemmas `lemma_load_block_byte_eq_arm64` and
  `lemma_load_block_byte_eq_avx2` compile but their bodies remain
  `admit ()` (the original byte_eq admits, moved up to the standalone
  layer). Net admit count unchanged at 5; admits relocated.

## Why this didn't close

### byte_eq body — 5-step instantiation chain unsolved

Several attempts at the standalone lemma body, all with `incomplete
quantifiers` at LOW rlimit (10–90 / 400). Failure mode is NOT a Z3
budget cliff — Z3 finishes search and concludes it can't prove. Each
attempt was structurally lacking a quantifier instantiation:

1. Minimal byte_eq body (one-liner: `if i < rate/8 then
   lemma_subslice_bytes_eq ...`) — same shape as Portable (passes
   at rlimit 200). FAILS in SIMD context.
2. With explicit `slices_same_len` forall trigger assert — failure
   shifts to next obligation.
3. With explicit precondition asserts (offset, rate, ii bounds) —
   sub-queries 1-141 pass, q142 (lemma call) fails.
4. With `--split_queries always` — q109 / q122 / q129 fail across
   attempts.

The 5 instantiations the proof requires:
- `arm64_lane/avx2_lane` SMTPat bridge fires on `lb_state.[ii]` and
  `state.[ii]` — needs `extract_lane` SMTPat to also fire on `lhs.[ii]`.
- `load_block`'s ensures forall instantiates at `ii`, requires good
  trigger on `state_future.[ii]` or `load_lane_u64 ... statei ...`.
- `load_lane_u64`'s `reveal_opaque` to unfold the lane definition into
  `get_lane_u64 statei lane ^. from_le_bytes(...)`.
- `createi_lemma` SMTPat for `xor_block_into_state`'s indexed access.
- `lemma_subslice_bytes_eq` for `blocks[l][offset..offset+rate]` vs
  `blocks[l][offset+8i..+8]`.

Per skill §1.5.1 — needed smtprofiling to identify which quantifier
isn't firing. Multiple iterations exhausted per-fn budget before that
diagnostic was attempted. The structural extraction (standalone lemma)
is the right architecture but each attempt to close it ran into a
different quantifier-instantiation issue, ratcheting through fixes
without convergence.

### Avx2 squeeze lemma — Fix 1 regression unclosed

`lemma_sq_lane_avx2_eq_squeeze_state` Fix-1-induced regression remains.
Step 2 (single reveal) sufficient for Arm64 (2 lanes) but not Avx2
(4 lanes). The 4-lane variant generates 2× the per-i ghost calls and
the cascade saturates 800 rlimit on the monolithic VC AND on split
retry sub-query 102.

This represents a separate cliff that Fix 1 unmasked. Closing it likely
requires either:
- Extracting the squeeze lemma's byte_eq into a standalone lemma (same
  structural-extraction approach as load_block byte_eq).
- Lifting reveal of `avx2_lane` to a stronger SMTPat that fires
  bidirectionally without ambient noise.
- Restoring the lemma's previous rlimit semantics by reducing the
  ghost-call set in byte_eq's body (minimised in this session — still
  failing).

## Key empirical findings (added to investigation note's evidence)

1. **Step 1 (opacity foundation) does close Keccakf modules cleanly**
   (concurrence with Track B prior session): Keccakf.Arm64 in 710 ms,
   Keccakf.Avx2 in 1.9 s on warm cache. The foundation is solid.

2. **Step 2 alone (KA.lemma_*_lane_unfold reveal in squeeze byte_eq)**:
   - Sufficient for Arm64 squeeze: 26 s, 161/800 rlimit.
   - INSUFFICIENT for Avx2 squeeze: cancels at 800/800 on sub-query 1
     after 155 s.
   The 2× lane difference matters; the 4-lane cascade is structurally
   harder than 2-lane.

3. **Step 3 (structural extraction)** — the byte_eq lemma can be
   architecturally extracted, but proving its body in isolation still
   hits the same instantiation issues (just at the lemma level instead
   of via the outer `Classical.forall_intro` Skolem). The Track B
   prior session's prediction that this would clear the path was
   **incorrect for the lemma body** — the next-down failures are not
   in the cascade-cliff family, they are structural Z3-instantiation
   gaps that need per-quantifier triggers, not a structural refactor.

## Recommended next steps for the next agent

1. **Run smtprofiling on `lemma_load_block_byte_eq_arm64`** with the
   admit removed, at low rlimit (200), with `--log_queries`. Identify
   the dominant quantifier that's NOT instantiating (likely the
   `load_block` ensures forall or the slices_same_len forall).
2. **Add a tighter SMTPat or explicit lemma**: depending on which
   quantifier blocks. The standalone lemma layer is the right
   architecture; just needs the precise instantiation hint.
3. **Avx2 squeeze regression — apply same structural-extraction
   pattern**: extract `lemma_sq_lane_avx2_eq_squeeze_state`'s byte_eq
   into a standalone top-level lemma parameterized by
   `i: nat{i < Seq.length out_l}`. This isolates the cascade.
4. **Consider stronger SMTPats**: add a multi-pattern SMTPat to
   `lemma_arm64_lane_eq_get_lane_u64` and the AVX2 mirror that fires
   on `(extract_lane lc state l).[i]` (the consumer pattern), so that
   the bridge fires when consumers index extract_lane results.

## File deltas in this session

- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Arm64.fst`:
  + opacity attribute on `arm64_lane`
  + new `lemma_arm64_lane_unfold` reveal helper
  + reveal calls threaded through all 7 `arm64_lc_*` lemma bodies
  + reveal call in `lemma_extract_lane_zero_arm64`'s aux
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Avx2.fst`:
  + opacity attribute on `avx2_lane`
  + new `lemma_avx2_lane_unfold` reveal helper
  + reveal calls threaded through all 7 `avx2_lc_*` lemma bodies
  + reveal call in `lemma_extract_lane_zero_avx2`'s aux
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.fst`:
  + reveal call in `lemma_arm64_lane_eq_get_lane_u64`'s body
  + new `lemma_load_block_byte_eq_arm64` standalone lemma (body: admit())
  + outer `lemma_load_block_eq_xor_block_into_state_arm64`'s byte_eq
    inner reduced to one-line call to standalone
  + `KA.lemma_arm64_lane_unfold` threaded into
    `lemma_sq_lane_arm64_eq_squeeze_state`'s byte_eq
  + push-options rlimit on squeeze lemma bumped 400 → 800 (still
    within absolute cap; not split_queries).
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.fst`:
  + same shape as Arm64 mirror
  + Avx2 squeeze lemma's byte_eq currently fails — Step 2 alone
    insufficient.
  + push-options rlimit on squeeze lemma raised to 800.
  + standalone lemma's body is admit() with FOLLOW-UP comment.

## Build artifacts

- Full sponge build with Step 1 + Step 2 + structural extraction:
  `/tmp/tb2-sponge-final.log` (Arm64 passes 26 s; Avx2 squeeze fails).
- Foundation Keccakf cold build: `/tmp/tb2-foundation.log`
  (13 min total wall, includes all deps; Arm64 = 710 ms, Avx2 = 1.9 s
  on warm cache).

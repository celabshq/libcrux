# Track B re-attempt status (sha3-proofs-track-b-2)

## 2026-05-24 — start
- Sub-task: skill loaded, baseline diagnosed, starting Step 1 (Fix 1 opacity)
- Blocker: none
- ETA: ~10 min for Step 1 foundation closure check

## 2026-05-24 — Fix 1 applied; foundation closes
- Sub-task: Fix 1 (opacity on KA.arm64_lane + KA.avx2_lane, reveal helpers,
  arm64_lc_* / avx2_lc_* lemma body reveals, lemma_*_lane_eq_get_lane_u64
  reveals in Sponge bridges) applied. Cold build of Keccakf.Arm64 + Avx2
  modules: exit 0, 13 min wall (cold cache, deps inclusive).
  Both .checked files present. Next: validate Sponge.{Arm64,Avx2} regression.
- Blocker: none
- ETA: ~15 min for sponge regression check (cache now warm)

## 2026-05-24 — Track B regression reproduced, Steps 2+3 applied
- Sub-task: Initial sponge build (Fix 1 only) confirms Track B's regression:
  arm64 squeeze lemma 29s (used 181/400 rlimit, OK); avx2 squeeze lemma
  FAILED — sub-query 1 cancels at 117s/600 rlimit; split-retry sub-query 99
  fails after 110s. Build exit 2. Total 6.5 min wall.
  Step 2 (companion fix): added KA.lemma_{arm64,avx2}_lane_unfold reveals
  into squeeze lemma byte_eq branches. Step 3 (structural extraction):
  added standalone top-level lemmas lemma_load_block_byte_eq_{arm64,avx2}
  taking concrete (i: nat{i < 25}); outer lemmas reduced to thin wrappers
  calling the standalone. Build #2 in-flight (already in Avx2.Load deps).
- Blocker: none — Steps 2+3 await build result.
- ETA: ~6 min for sponge build to complete (post Avx2.Load deps)

## 2026-05-24 — Final state: 0 admits closed, architecture improved
- Sub-task: Multiple iteration cycles attempted to close
  lemma_load_block_byte_eq_{arm64,avx2}.fst body. Each attempt failed with
  "incomplete quantifiers" at low rlimit (41-90 / 400) — NOT cliff, structural.
  Even the MINIMAL byte_eq body (one-liner: if i < rate/8 then
  lemma_subslice_bytes_eq) fails — same shape as Portable (working) but
  with SIMD layer added. Per the no-new-admits policy, reverted to keep
  the standalone lemma's body as admit() with FOLLOW-UP comment. Net admit
  count: unchanged at 5 (admit moves from inner-byte_eq to standalone
  lemma but counted as same). Architecture improvement: structural
  extraction in place for next session.
- Blocker: byte_eq body has 5-step instantiation chain (arm64_lane SMTPat
  + load_block forall + load_lane_u64 reveal + createi_lemma + subslice
  bytes_eq) — Z3 doesn't find the right instantiation order. Per skill
  §1.5.1, would need smtprofiling on the failing query to identify the
  blocking quantifier. Out of per-fn budget.
- ETA: NA — handing off via rollup

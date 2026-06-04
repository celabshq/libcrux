# squeeze4 Phase 2 — status (2026-05-31)

Goal: close the squeeze4 body (last sha3 admit). Mechanical N=2→N=4 transfer of
the verified squeeze2 architecture.

## Done
- Wrote `EquivImplSpec.Sponge.Avx2.SqueezeDriver.fst` (N=4 mirror of
  `Arm64.SqueezeDriver.fst`): squeezed_upto opaque pred + lane-indep helpers
  (iterate_keccak_f_zero, squeezed_upto_full, squeeze_length, exact_multiple,
  blocks_rate_split) + first/one/mid/tail step lemmas + sq_lane↔f_squeeze4
  bridge + first/mid/tail driver wrappers. Added `lemma_squeeze_one_step_avx2`
  in this module (avx2 Steps has only the per-block lemma, unlike arm64 Steps).
- Split `squeeze4` in `src/generic_keccak/simd256.rs` into `squeeze4_blocks`
  (engine, --fuel 0, squeezed_upto ensures) + `squeeze4` composer; removed
  `--admit_smt_queries true`. (Not yet re-extracted.)

## Blocker under investigation
- First isolated build of SqueezeDriver: all lemmas verified EXCEPT
  `lemma_squeeze_one_step_avx2` query 81 — the `i *! rate` overflow /
  sq_lane precondition at the `lemma_squeeze_block_avx2` call site SATURATED
  (rlimit 400.000, 53 s).
- Measured arm64 `lemma_squeeze_one_step_arm64` hint-free: identical structure,
  every sub-query uses <4 rlimit. So this is NOT 2× N=4 scaling — it's an
  avx2-specific cascade poisoning a trivial check (heavy requires: recursive
  squeeze + per-byte forall; no using_facts_from filter in this module).
- Fix in flight: harden the body — name `start = i *! rate`, pre-establish
  `v start == v i*v rate` + bound asserts before the call (deterministic, per
  squeeze2 plan lesson). Isolated `--admit_except` test running.

## Progress (resolved)
- Hardening (name `start = i *! rate`, pre-assert bounds before the
  `lemma_squeeze_block_avx2` call) CLOSED query 81. Root cause: heavy requires
  (recursive squeeze + per-byte forall, no using_facts_from filter) poisoned the
  trivial `i*!rate` overflow/sq_lane-precond check at the call site; arm64 N=2
  one_step is trivial (<4 rlimit) hint-free, so this was an avx2 call-site
  cascade, not 2× scaling.
- **Full SqueezeDriver build GREEN** (no --admit_except): "All verification
  conditions discharged", EXIT 0, no Error 19 / saturation (270 s).
- `bash hax.sh extract` (sha3 entrypoint; ml-kem/aesgcm have hax.py, sha3 has
  hax.sh) re-extracted cleanly: git diff = ONLY Simd256.fst + simd256.rs (no
  drift in platform/core-models/intrinsics/secrets). Extracted Simd256.fst has
  no admit_smt_queries; squeeze4_blocks + squeeze4 with --fuel 0 +
  using_facts_from filter; all SqueezeDriver citations wired.
- Audit: zero active assume val / admit() / admit_smt_queries in tracked sha3
  F*; no ADMIT_MODULES. SqueezeDriver.fst is the new untracked file.

## DONE — squeeze4 body closed (zero sha3 admits)
- **Full Simd256.fst gate GREEN** (no --admit_except): "Verified module:
  Libcrux_sha3.Generic_keccak.Simd256", "All verification conditions discharged",
  EXIT 0, no Error 19 / saturation / Subtyping (205 s). This re-verified
  SqueezeDriver from source too (cache blocked → no .checked). The composer did
  NOT saturate — the equality-form arithmetic + opaque squeezed_upto held at N=4.
- squeeze4's strengthened ensures is UNCHANGED from Phase 1, so the Phase-1
  `lemma_squeeze4_avx2` / `lemma_keccak4_avx2` consumers in Avx2.API.fst remain
  valid (body closed, post unchanged).

## cargo test --features simd256 — PLATFORM-BLOCKED (pre-existing, not my change)
- Fails to compile `crates/utils/intrinsics/src/avx2.rs` (108 errors:
  `_mm256_permute2x128_si256`, `_mm_clmulepi64_si128`, `_mm_aesenc_si128`, …) —
  x86 AVX2/AES-NI intrinsics that do not exist on this arm64 (Apple Silicon)
  host. Fails inside libcrux-intrinsics BEFORE sha3 is reached. My diff touches
  only sha3 (simd256.rs + Simd256.fst), never the intrinsics crate → pre-existing
  platform limitation, must be run on x86/CI.
- Functional confidence on arm64 instead: (1) hax extraction recompiled the sha3
  Rust (stub backend) cleanly; (2) the split is behavior-preserving by
  construction (identical squeeze4/keccakf1600 call sequence; only proof-only
  fstar! lemmas + the squeeze4_blocks function boundary added); (3) the F* proof
  is the functional-equivalence guarantee, and it is green.

## Verification helper
- `/tmp/sha3_fstar.sh <abs.fst> [extra flags]` (canonical includes; judge by
  EXIT 0 + "All verification conditions discharged"; cache writes blocked by
  corrupt Core_models.Num.fst.checked — .checked never written, deps re-verify).

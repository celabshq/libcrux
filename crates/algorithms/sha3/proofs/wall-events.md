# SHA-3 Proof Wall Events

Cross-cutting events that don't fit a single milestone row.

## 2026-05-01: cross-branch-audit `rust-spec`

**Event-type:** cross-branch-audit
**Trigger:** parallel agent flagged that the sibling repo
`~/libcrux-trait-opacify` has a `rust-spec` branch (27 commits,
2026-03-10 to 2026-03-23, pre-fork) with earlier F* equivalence
proofs for sha-3 — possibly cherry-pickable to close
`lemma_squeeze2_arm64`.

**Files audited:**
- `specs/sha3/proofs/fstar/extraction/Sha3_Equivalence.fst`
  (rust-spec, Portable, 991 LOC, 61 lemmas, 14 admits)
- `specs/sha3/proofs/fstar/extraction/Sha3_Equivalence_Avx2.fst`
  (rust-spec, Avx2 stub, 68 LOC, 0 lemmas, 1 admit)
- `specs/sha3/proofs/fstar/extraction/Sha3_Equivalence_Neon.fst`
  (rust-spec, Neon, 740 LOC, 55 lemmas, 10 admits — primary candidate)

**Verdict:** **SUPERSEDED.  Do not cherry-pick.**

**Findings:**
- rust-spec's `lemma_neon_keccak2_lane0_equiv` matches our admitted
  `lemma_squeeze2_arm64` in spirit — but it's also `assume val` on
  rust-spec, with `ensures True` (real statement blocked by the same
  fold_range normalization issue).  It targets `keccak2` (full
  driver), not `squeeze2` in isolation.
- 15 proven (`let`) keccakf-level lemmas in rust-spec
  (`lemma_neon_zero_lane`, `lemma_neon_xor5_lane`, ...,
   `lemma_neon_keccakf1600_equiv`, ...) all reproduced in current
  `EquivImplSpec.Keccakf.Arm64.fst` / `EquivImplSpec.Sponge.Arm64.fst`
  under different names.
- 7 `assume val` intrinsic lane-wise primitives in rust-spec
  (neon_lane, neon_vdupq_n_u64_lane, ...) all reproduced in current
  `Libcrux_intrinsics.Arm64_extract` SMTPats.
- Portable side has no utility lemma missing from current
  `Libcrux_sha3.Proof_utils.Lemmas.fst` / `Hacspec_sha3.Sponge.Lemmas.fst`.
- Avx2 stub is fully superseded by current
  `EquivImplSpec.Keccakf.Avx2.fst` (proven, milestone row 3).

**Action:** none.  Continue with byteform-shaped closure of
`lemma_squeeze2_arm64` per `BRIEF_squeeze_steps.md` + the new
`EquivImplSpec.Sponge.Portable.Steps.lemma_squeeze_one_step_portable`
(committed 2026-05-01 at `f11e50419`).

# Plan: close `lemma_squeeze4_avx2` — the last sha3 admit

## STATUS (2026-05-31)
- **Phase 1 DONE — committed `1723b46a9`.** `lemma_squeeze4_avx2` is a real `let`
  (proven), `lemma_keccak4_avx2` closes, `Avx2.API.fst` verifies EXIT 0. squeeze4's
  ensures is strengthened to the 4-lane spec; its body is admitted
  (`Simd256.fst:270` `--admit_smt_queries true`) — the **sole remaining sha3 admit**.
- **Phase 2 = PICKUP (fresh session).** Close the squeeze4 body per §"Phase 2" below.
  Start by removing `--admit_smt_queries true` from `squeeze4`'s
  `fstar::options` in `src/generic_keccak/simd256.rs` and following the split.

## Context

Post-commit `94cc7680b`, the **only** remaining assume/admit in the sha3 crate is
`assume val lemma_squeeze4_avx2` (`EquivImplSpec.Sponge.Avx2.API.fst:87`) — the
N=4 AVX2 driver lemma. `lemma_keccak4_avx2` (same file, real `let`) already
consumes it 4×, so closing it closes the entire AVX2 X4 chain.

**`squeeze4` is at squeeze2's exact starting point**: `src/generic_keccak/simd256.rs:178`
is monolithic (blocks==0 / first-block + `1..blocks` loop / tail, all inline),
ensures only length-preservation, `--z3rlimit 600 --split_queries always`.

**The lower-level AVX2 lemmas already exist and are proven** (verified by audit —
zero admits in these):
- `avx2_sc_store_block` (`Sponge.Avx2.fst:478`) — first-block per-lane store, no keccakf. (= arm64 `arm64_sc_store_block`)
- `sq_lane_avx2` (`Sponge.Avx2.fst:41`) + `lemma_sq_lane_avx2_eq_squeeze_state` (:424) — per-lane projection + byteform bridge.
- `lemma_squeeze_block_avx2` (`Sponge.Avx2.Steps.fst:131`) — mid block (keccakf+squeeze). (= arm64 `lemma_squeeze_one_step_arm64`)
- `lemma_squeeze_last_avx2` (`Steps.fst:171`) — tail block. (= arm64 `lemma_squeeze_last_arm64`)
- `avx2_lane` + `lemma_avx2_lane_unfold` (= `I.get_lane_u64x4`) (`Keccakf.Avx2.fst:59`). (= arm64 `arm64_lane`/`get_lane_u64`)

So the work is a **mechanical N=2→N=4 transfer of the squeeze2 driver architecture**
(`EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst` + the `Simd128.squeeze2` split),
with the squeeze2 pitfalls known up front. No new lower-level math.

## Phase 1 — Top-down spike: strengthen the post, prove the driver lemma (de-risk)

Goal: replace `assume val lemma_squeeze4_avx2` with a real `let` proven from
`squeeze4`'s strengthened (but body-admitted) ensures. This validates the post is
*sufficient* for the consumer before paying for the body proof
(memory `feedback_drive_to_top_spike`). Net admit count is unchanged (1 `assume val`
→ 1 admitted body), but the structure becomes correct and the driver lemma + keccak4
chain go green.

1. **`squeeze4` (simd256.rs)**: add `--admit_smt_queries true` to its
   `fstar::options`; **strengthen `ensures`** with the 4-lane functional spec,
   guarded by `outlen < MAX-200`, mirroring `lemma_squeeze4_avx2`'s ensures —
   for `l = 0..3`: `out{l}_future == Hacspec_sha3.Sponge.squeeze outlen
   (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4) KA.lc_avx2 s.f_st l) rate`.
   (Keep length-preservation conjuncts.)
2. **`lemma_squeeze4_avx2` (Avx2.API.fst)**: `assume val` → `let`, body just calls
   `Simd256.squeeze4 rate s out0 out1 out2 out3` and selects lane `l` from its
   ensures (a thin consumer — mirror how arm64 `lemma_squeeze2_arm64` in
   `Driver.fst` reads `Simd128.squeeze2`'s ensures).
3. **Verify** (fstar.exe direct, §Verification): `Avx2.API.fst` — `lemma_squeeze4_avx2`
   and `lemma_keccak4_avx2` close. If the post shape is wrong, this fails cheaply
   here, not after the body proof.

## Phase 2 — Close the `squeeze4` body (squeeze2 playbook at N=4)

1. **Split `squeeze4` (simd256.rs)** — remove `--admit_smt_queries`:
   - `squeeze4_blocks` (private): first block (no keccakf) + `1..blocks` loop only.
     `--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always
     --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane'`.
     Ensures (guarded `outlen<MAX-200`): for `l=0..3`, `extract_lane … s_future.f_st l
     == iterate_keccak_f (blocks-1) (extract_lane … s.f_st l)` ∧ `squeezed_upto out{l}_future
     (squeeze outlen (extract_lane … s.f_st l) rate) (blocks*RATE)`. Base case cites
     `lemma_iterate_keccak_f_zero` ×4.
   - `squeeze4` composer: `blocks==0` / tail (`last<outlen`) / exact (`else`) branches,
     each closing via the driver wrappers + `lemma_squeezed_upto_full` so no VC
     carries both the loop and the byteform `squeeze` equality.
2. **New manual helper `EquivImplSpec.Sponge.Avx2.SqueezeDriver.fst`** (untracked,
   mirror `Arm64.SqueezeDriver.fst` at N=4, built on the existing AVX2 Steps +
   `avx2_sc_store_block`):
   - `squeezed_upto` opaque pred + `lemma_squeezed_upto_full` + `lemma_squeeze_length`
     + `lemma_iterate_keccak_f_zero` + `lemma_exact_multiple` + `lemma_blocks_rate_split`.
     These are **lane-independent** — either duplicate from `Arm64.SqueezeDriver`
     (simplest, lowest-risk first pass) or factor into a shared module both import
     (cleaner; touches arm64 → re-verify). **Recommend duplicate now, factor later.**
   - first/mid/tail **step** lemmas (`l < 4`): wrap `avx2_sc_store_block` /
     `lemma_squeeze_block_avx2` / `lemma_squeeze_last_avx2` into `squeezed_upto`
     terms; confine `reveal_opaque squeezed_upto` + the per-byte `aux` to each body.
   - first/mid/tail **driver** wrappers (all 4 lanes at once): package
     `outputs = [out0;out1;out2;out3]`, bridge `f_squeeze4 ↔ sq_lane_avx2` (analog of
     `lemma_sq_lane_is_f_squeeze2`), call each step lemma ×4.
3. **Wire composer citations** (mirror the verified `Simd128.squeeze2`): per-branch
   `lemma_exact_multiple` (else) / `lemma_div_mod` + `lemma_blocks_rate_split` (tail),
   `lemma_squeeze_length` ×4 before the `lemma_squeezed_upto_full` ×4.

## Squeeze2 lessons to apply FROM THE START (do not re-discover)

- **`--fuel 0`** on both `squeeze4_blocks` and `squeeze4` + cite
  `lemma_iterate_keccak_f_zero` for the base case — stops the `iterate_keccak_f`
  recursive cascade (was squeeze2's original query-62 saturation).
- **`lemma_squeeze_length`** — the `using_facts_from '* -…squeeze'` exclusion drops
  `squeeze`'s return-type length, so `lemma_squeezed_upto_full`'s equal-length premise
  needs it re-exposed.
- **Equality-form arithmetic** (`feedback_equality_beats_strict_ineq_composer`): else
  branch uses `lemma_exact_multiple` (concludes `blocks*rate == outlen` *directly*);
  tail uses `lemma_blocks_rate_split` (hands Z3 the **equalities** `last==blocks*rate`,
  `blocks*rate+rem==outlen`, `rem<rate` so it substitutes). **Do NOT** hand the
  composer strict inequalities (`blocks*rate < outlen`) — they saturate rlimit 400
  in the heavy composer context.
- **Harden the per-byte `aux` `v b == v blocks`** (in the tail step lemma) up front:
  `assert (v rate > 0)`; `small_div` + `lemma_div_plus`; `assert (k/v rate == v blocks)`;
  machine→math bridge `assert (v b == v kk / v rate)`. (squeeze2 regressed here when
  a sibling lemma perturbed Z3's search; the explicit chain is deterministic.)
- **Per-lane seeds ×4**: the byte-eq bridge needs `get_lane_u64x4 v (mk_usize j)`
  mentioned for `j=0..3` per vector (`feedback`-pattern "Per-lane get_lane_u64 seeds";
  N=4 ≈ 2× the N=2 seed set, closes ~rlimit 176/400, under cap). Guard predicate
  bodies with `l < 4` (`get_lane_u64x4` requires it).
- **Test the COMPOSER via the FULL build**, not `--admit_except squeeze4` — the
  composer saturation only appears in full-module context (`--admit_except` admits
  siblings and passes misleadingly; this cost two wasted full builds on squeeze2).

## Verification (run locally via `fstar.exe` direct — proxy sessions are flaky here)

Derive includes once: `make --dry-run` in `proofs/fstar/extraction` (or reuse
`/tmp/sha3_incs.json`). Cache writes are blocked project-wide by the corrupt
`Core_models.Num.fst.checked` — **judge by EXIT 0 + "All verification conditions
discharged" + absence of `Error 19`/`used rlimit 400`**, not by `.checked`.

- **Phase 1 gate**: `fstar.exe … EquivImplSpec.Sponge.Avx2.API.fst` → green
  (`lemma_squeeze4_avx2` + `lemma_keccak4_avx2` close from the admitted-body post).
- **Phase 2 gates**:
  1. `fstar.exe … EquivImplSpec.Sponge.Avx2.SqueezeDriver.fst` **hint-free**, twice
     (different `--z3seed`) — confirm the new lemmas + the hardened `aux` are
     deterministic.
  2. **Full** `fstar.exe … Libcrux_sha3.Generic_keccak.Simd256.fst` (NO
     `--admit_except`) — the Phase-8 gate (this is where composer saturation shows).
     The Simd256 module is large (≈ Avx2 module size); budget ~15–20 min/run.
  3. `cargo test --features simd256`.
  4. Re-run the admit audit — expect **zero** remaining assume/admit in sha3.

## Risks / notes
- N=4 ≈ 2× the per-query SMT cost of N=2 but stays under rlimit 800 (per the
  load_block N=2→N=4 measurements). No structural cliff expected.
- `Simd256.fst` full builds are slow; minimize iterations by closing
  `SqueezeDriver` (fast, isolated) first, then one full Simd256 gate.
- New file `Avx2.SqueezeDriver.fst` is untracked — note at commit (like
  `Arm64.SqueezeDriver.fst`).
- Optional cleanup: stale comment `Arm64.Driver.fst:42` still says
  `lemma_squeeze2_arm64` is "ADMITTED" (it's a real `let` at :111).

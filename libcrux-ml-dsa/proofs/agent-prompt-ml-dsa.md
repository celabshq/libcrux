# Agent prompt — ML-DSA milestone push

Paste this into a fresh Claude Code session opened in
`~/libcrux-ml-dsa-proofs/libcrux-ml-dsa` (auto mode recommended).

This prompt is informed by two prior audits whose synthesis is in
`proofs/sprint-learnings.md`. The cross-audit consensus that the
biggest gap is "spec-module design" was **based on a stale view** —
in fact `specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.*`
already provides a substantial extracted spec layer (see
"Existing spec inventory" below).  The real gap is **wiring impl
ensures to the existing Hacspec functions**, not designing the spec
from scratch.

---

You are a single-lane agent for the libcrux-ml-dsa F\* verification effort.
Branch: `ml-dsa-proofs` (current tip — see `proofs/post-merge-handoff.md`).
Goal: close named milestones in `proofs/proof_milestones.md`.

## Existing spec inventory (DO NOT redesign — survey before wiring)

  At `specs/ml-dsa/proofs/fstar/extraction/`:
    - `Hacspec_ml_dsa.fst` — top-level umbrella.
    - `Hacspec_ml_dsa.Ml_dsa.fst` (634 lines) — defines
      `keygen_internal`, `sign_internal`, `verify_internal`. This
      IS the FIPS-204 algorithmic spec.
    - `Hacspec_ml_dsa.Ntt.fst` (189 lines) — forward NTT spec
      with FIPS 204 zetas table, `bit_rev_8_`, etc.
    - `Hacspec_ml_dsa.Encoding.fst` (1477 lines) — encoding spec
      (T0/T1/error/gamma1/commitment/signature/signing_key/vk).
    - `Hacspec_ml_dsa.Matrix.fst` (43 lines).
    - `Hacspec_ml_dsa.Sampling.fst` (350 lines).
    - `Hacspec_ml_dsa.Hash_functions.fst` (85 lines).
    - `Hacspec_ml_dsa.Polynomial.fst` (434 lines),
      `Hacspec_ml_dsa.Arithmetic.fst` (161),
      `Hacspec_ml_dsa.Parameters.fst` (183).

  At `specs/ml-dsa/proofs/fstar/commute/`:
    - `Hacspec_ml_dsa.Commute.Chunk.fst` (762 lines) — per-lane
      commute lemmas (`lemma_reduce_lane_commute`,
      `lemma_power2round_lane_commute`,
      `lemma_decompose_bridge`, `lemma_compute_hint_bound`,
      `lemma_mont_mul_bound_and_mod_q`, etc.). These already
      bridge i32-level operations between impl and spec.

  Already wired: `src/simd/avx2.rs` and `src/simd/portable.rs`
  cite the per-lane commute lemmas in their post-conditions.
  Below-trait NTT bridges are partly in place via
  `Hacspec_ml_kem.Spec` import paths in the include list.

  The `Hacspec_ml_kem.Commute.Chunk.fst` reference IS NOT a
  typo — both ml-kem and ml-dsa share some chunk-level reasoning
  via the include path `libcrux-ml-kem/proofs/fstar/spec`.

## Recently closed (do not redo)

  - **Multi-agent parallel sprint** (2026-04-30, three worktrees):
    + `94e679871` — `Encoding.Error.deserialize_to_vector_then_ntt`
      body admit closed via uniform `is_i32b 11` bound (covers both
      eta=2 → [-5,2] AND eta=4 → [-11,4]) instead of eta-case-split.
      rlimit 80 default, no split_queries needed. Innovation: avoids
      quantifier-trigger explosion when `eta` is symbolic.
    + `ea7ae0e92` — `Simd.Avx2.Rejection_sample.{Less_than_eta,
      Less_than_field_modulus}` lifted out of `ADMIT_MODULES`. Pres:
      `(ETA==2||4) && input.len()=={4,24} && output.len()>=8`.
      Length-pres ensures added to `mm_storeu_si128_{i32,u8}` in
      `crates/utils/intrinsics/src/avx2.rs`. `assume (count_ones (good
      & 0xF) <= 4)` mirrors libcrux-ml-kem AVX2 sampler idiom.
    + `484fb664a` — `Encoding.Signature.serialize` SCAFFOLDING ONLY
      (body admit retained). Added recursive F* spec helpers
      `count_row_ones` + `count_total_ones` and pre `count_total_ones
      $hint <= v $max_ones_in_hint`. Body close needs helper-split
      mirror of PR 1348 deserialize closure.
    Baseline after merge: 78 invoked, [CHECK]=62, [ADMIT]=16,
    0 F* errors.
  - **NTT port plan landed** (`proofs/ntt-port-plan.md`,
    2026-04-30). Read-only review of `~/libcrux-trait-opacify`'s
    NTT proof machinery + reusability matrix + 14-step ordered
    porting plan + recommended FIRST commit (chunking lemma
    `simd_units_to_array` in `Hacspec_ml_dsa.Commute.Chunk.fst`,
    ≤3hr, no Z3-divergence risk). Critical-path lemma identified.
    Total estimate: ~37 agent-hours for full forward+inverse close
    (milestones 1, 2, 5, 6, 7).
  - **Spec inventory + gap analysis pass on `proof_milestones.md`**
    (`75f26ee10`, 2026-04-30). Replaces the stale "many sprints —
    Hacspec_ml_dsa.Ntt does not exist" framing. After audit: 0 of 25
    milestone rows are spec-gated; almost everything is ⛓️
    wiring-gated, with NTT/Invntt rows 📦 partial-spec (per-layer
    commute lemmas TBD). Doc-only commit. **Closes Priority §0** —
    next agents start at §1 or remaining §2 items.
  - **`Specs.Simd.Portable.Sample.fst`** lifted out of ADMIT_MODULES
    (`905cfc8e9`, 2026-04-30). Loosened `< max_usize` to `<= max_usize`
    on `Spec.MLDSA.Math.rejection_sample_{field_modulus,eta_2,eta_4}`
    and replaced overflow-prone `len()*2 <= u32::MAX` with
    `len() <= 2_147_483_647` in `src/specs.rs` eta_{2,4} pres.
    Module verifies in 925ms. **Closes the "Spec dispatcher (1)"
    bullet under Priority §2.** ADMIT_MODULES now 19 entries (was 20).
  - **Makefile flipped to ml-kem allowlist style** (`74922609a`,
    2026-04-30). `proofs/fstar/extraction/Makefile` now lists 19
    explicit `ADMIT_MODULES` entries in 3 remaining categories
    (12 user-API + 4 Samplex4 + 3 AVX2 Rejection_sample); every
    other `.fst` is verified by full SMT. Newly-extracted files
    default to verified, forcing an explicit Makefile entry to admit.
  - **`Encoding.Verification_key.generate_serialized`** body admit
    closed (`5d32e16df`). Pattern: mirror `Signing_key.generate_serialized`
    + `assert_norm` for the `RING_ELEMENT_OF_T1S_SIZE` constant chain.
  - **`Constants.Ml_dsa_{44,65,87}_.fst`** promoted to verified
    (`74922609a`). Pure const definitions; no source change required.

## Deferred with concrete plan (do not start blind)

  - **`Encoding.Signature.serialize`** body close — scaffolding
    landed (`484fb664a`): `count_total_ones` spec helper + pre
    `count_total_ones $hint <= v $max_ones_in_hint`. Body still
    admits because (a) inner `gamma1::serialize` needs polynomial
    bound `is_pos_array_opaque (pow2 gamma1_exponent - 1)` on
    `signer_response[i].f_simd_units` (caller-established, not
    yet exposed as `serialize` precondition) and (b) `count_*`
    monotonicity lemmas needed. Helper-split mirror PR 1348
    `validate_hint_rows`/`write_hint_rows` is the path forward.
    Estimate: 2-3 hr.

## Priority order

  **0. ✅ DONE (`75f26ee10`).** Spec inventory + gap analysis pass
     landed on `proofs/proof_milestones.md`. After audit, 0 of 25
     rows are 🆕 spec-gated; the gating column is now
     ⛓️ wiring-gated / 📦 partial-spec. Read the refreshed
     `proof_milestones.md` for the per-row picture.

  **1. Wire `Ml_dsa_generic.{generate_key_pair_internal,
     sign_internal, verify_internal}` ensures to
     `Hacspec_ml_dsa.Ml_dsa.{keygen_internal, sign_internal,
     verify_internal}`.**  These three Hacspec functions ALREADY
     EXIST.  Wiring them gives the strongest "this is FIPS 204"
     statement available without further spec work.  Each side is
     currently body-admitted; the wiring task is to add the
     `ensures` block that cites the Hacspec function, then close
     the body via composition through the (already verified) layer
     beneath. Per fn, est. 2-4 hr.

  **2. Drive `ADMIT_MODULES` to zero (parallel-friendly with the rest).**
     `proofs/fstar/extraction/Makefile` lists 19 explicit admits in
     3 remaining categories (the "Spec dispatcher (1)" group was
     closed in `905cfc8e9`). Source-side reasons documented inline.
     Pick the cheapest unblocked group:
       - **Samplex4 (4)** — needs trait-method panic-freedom on the
         X4 Xof hash functions. Probably 2-3 hr per dispatcher.
       - **AVX2 Rejection_sample.{Less_than_eta, Less_than_field_modulus}
         (2)** — Step 13 Track A AVX2 closure shape. ~1-2 hr each.
       - **Shuffle_table (1)** — DON'T attempt; needs a hax-proof-libs
         detour. See AP-4 below + commit `9da124ba5`.
       - **User-facing API wrappers (12)** — gated on (1) — they
         thread through `Ml_dsa_generic.*` and become correct-by-
         construction once that layer's bodies close.

  **3. Wire encoding wrapper ensures to
     `Hacspec_ml_dsa.Encoding.*`.**  The Hacspec encoding spec is
     1477 lines and covers T0/T1/error/gamma1/commitment/signature/
     signing_key/vk.  Each impl wrapper in `src/encoding/*.rs` has
     bounds-only or panic-only ensures today; replace them with
     citations to the Hacspec spec and prove the wiring.  Per
     wrapper, est. 1-2 hr (8 wrappers × ~12 hr total).

  **4. Wire NTT/Invntt wrapper ensures to
     `Hacspec_ml_dsa.Ntt.{ntt, invert_ntt_montgomery, ...}`.**
     Hacspec_ml_dsa.Ntt.fst already provides the FIPS 204 zetas
     table + the spec-level NTT/INTT. Per-lane bridges in
     `Hacspec_ml_dsa.Commute.Chunk` are partly in place. The
     remaining work is the per-layer SIMD chunking lemma (analogous
     to ML-KEM's `lemma_inv_ntt_layer_<N>_step_to_hacspec`).
     ~2-3 sprints across all layers + their inverses.

  **5. Stretch: ml-dsa-generic correctness composition.** Once
     (1)+(3)+(4) land, the user-facing API wrappers (currently in
     ADMIT_MODULES) become wiring-only. Many sprints, but the
     individual steps are mechanical compositions.

## Anti-patterns to avoid (cross-audit lessons)

  **AP-1 Big-axiom-bridge designs** — don't write a general
     "axiom-bridge" connecting the impl to the spec layer in one
     theorem. Pick one width / one variant (e.g. ML-DSA-44 first),
     prove it, generalize after. Per cross-audit: ML-KEM's
     `Hacspec_ml_kem.Serialize.HacspecBridge` was abandoned after 11
     commits; the per-width `panic_free` strategy is what produced
     gains.

  **AP-2 Defining ensures without verifying the spec exists** —
     bounds-only ensures are sometimes a "spec gap", sometimes
     just a "wiring gap". Before adding new `Hacspec_ml_dsa.*`
     definitions, **grep first**:
       `grep -rn "<spec_fn_name>" specs/ml-dsa/proofs/fstar/`.
     The spec layer is much larger than the earliest agent prompts
     assumed (see "Existing spec inventory"). Path: grep for spec
     → if missing, design spec → wire ensures → prove. If spec
     already exists, skip straight to wiring.

  **AP-3 GLOBAL `reveal_opaque (\`%P) (P)` in loop bodies (Rule SD4)**
     — ML-KEM saw a 153 s top-1 wall caused by one such line; fix
     dropped to 2.1 s. Use targeted form `reveal_opaque (\`%P) (P
     arg1 arg2)` or just an `assert (P args)` first.

  **AP-4 Don't fight `bits USIZE`.** The hax proof-libs `.fsti` keeps
     `bits USIZE` opaque; `assert_norm` does not unfold it either.
     If a function shifts on a usize, either bound the shift amount
     tighter (e.g. `< 8`) so the obligation falls into a different
     proof path, or leave the module admitted rather than burn time
     on a proof-libs detour. Tried+reverted in `9da124ba5`.

  **AP-5 `assert_norm` for arithmetic constant chains.** When a
     constant extracts via a chain that includes a subtraction step
     (e.g. `BITS_IN_UPPER_PART_OF_T = FIELD_MODULUS_MINUS_ONE_BIT_LENGTH
     - BITS_IN_LOWER_PART_OF_T`), plain `assert (v $C == K)` will
     not discharge under `fuel 0`; use `assert_norm`. Shorter chains
     without subtraction (e.g. `T0_SIZE = 13 * 256 / 8`) work with
     plain `assert`. See commit `5d32e16df`.

  **AP-6 NEVER bump `--z3rlimit` above 800 (≤ 400 with `--split_queries
     always`).** High-rlimit proofs are flaky: Z3 perf at high rlimit
     is non-monotone, so a proof passing at 1500 may fail at 1200 and
     pass again at 800. Bumping rlimit past the cap masks structural
     problems instead of fixing them. Default is `--z3rlimit 200`
     (R4); raise only after profiling shows a single hot query
     genuinely needs more budget.
     - With `--split_queries always`, the cap drops to 400 *per split*
       — each sub-query gets its own budget, so 400 per split is a much
       larger total than 400 monolithic. If a split proof needs > 400
       per query, the split granularity is wrong (factor differently
       or un-split).
     - When a proof is failing, the answer is NEVER "bump rlimit past
       the cap." Use AP-3 (targeted `reveal_opaque`), drive-to-the-top
       spike, factoring into helper lemmas, opacification, or accept
       `verification_status(lax)` until a structural fix lands.
     - When touching legacy code with rlimit > cap: reduce if possible;
       otherwise leave the legacy value but flag it in the commit
       message as needing structural cleanup. Don't *increase* an
       existing high value.

## Hard rules

  R1 Branch `ml-dsa-proofs` directly. User merges to origin manually.
  R2 No NEW broad admits. Adding to `ADMIT_MODULES` is allowed only
     for newly-extracted files that genuinely cannot verify yet, and
     must come with a one-line reason in the Makefile comment block.
  R3 No new axioms unless absolutely necessary. File as SIDEWAYS in
     `MLDSA_STATUS.md` + commit message.
  R4 `ulimit -v 8388608`. F\* `--z3rlimit ≤ 800` (≤ 400 with
     `--split_queries always`; see AP-6). Default `--z3rlimit 200`.
  R5 Inner edit-check: `make check/<Mod>.fst` from
     `proofs/fstar/extraction/`. Cap 20 min/attempt.
  R6 Re-record hints + touch unchanged `.checked` after extract — use
     the helper script (see "Per-build hygiene" below); do NOT
     re-roll the shell.
  R7 SIMD trait FROZEN unless adding NEW posts (per cross-audit AP-2).
  R8 After each milestone: regenerate `proofs/verification_status.md`,
     update `proof_milestones.md` status, commit prefix `agent-mldsa:`.

## Workflow

  1. Read `proofs/post-merge-handoff.md` (current state + tip),
     `proofs/proof_milestones.md` (TODO list), and
     `proofs/sprint-learnings.md` (cross-audit synthesis).
  2. Pick first milestone — start with Section 0 (spec inventory)
     unless that has already been landed; otherwise pick from
     Section 1 (Ml_dsa_generic wiring) or Section 2 (cheapest
     admit closure).
  3. Iterate `make check/<Mod>.fst` until clean.
  4. `python3 proofs/generate_verification_status.py` to refresh report.
  5. Update `proof_milestones.md` row status + commit SHA.
  6. Commit. Move to next.
  7. Cap: 3-4 milestones or 4 hours total.

## Per-build hygiene (paste-ready)

  ```bash
  cd ~/libcrux-ml-dsa-proofs/libcrux-ml-dsa
  proofs/agent-status/touch-unchanged-checked.sh snapshot
  JOBS=2 ./hax.sh extract
  proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
  JOBS=2 ./hax.sh prove 2>&1 > /tmp/baseline.log
  grep -E "Modules invoked|F\* errors" /tmp/baseline.log
  # Expect on a clean tree: ~78 invoked, [CHECK]≈58, [ADMIT]≈20, 0 F* errors.
  ```

  If the baseline drifts (CHECK count drops or errors appear), the
  first task is to find what regressed before editing.

## Deliverable

End-of-session report (≤ 200 words):
  - Milestones closed (✅) with commit SHAs.
  - Modules removed from `ADMIT_MODULES` (if any) and why.
  - Hacspec spec functions newly cited from impl `ensures` —
    fn-name, file path, commit SHA.
  - New `Hacspec_ml_dsa.*` definitions added (only if grep
    confirmed they were genuinely missing) — file path + summary.
  - Final commit SHA on `ml-dsa-proofs`.
  - Next-priority recommendation.

DO NOT push to origin. DO NOT touch `~/libcrux-trait-opacify` or
`~/libcrux-sha3-focused`.

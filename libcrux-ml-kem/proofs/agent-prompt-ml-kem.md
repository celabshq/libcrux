# Agent prompt — ML-KEM continuation (post-2026-04-30c session)

Paste this into a fresh Claude Code session opened in
`~/libcrux-trait-opacify/libcrux-ml-kem` (auto mode recommended).

You are a single-lane agent for the libcrux-ml-kem F\* verification
effort.  Branch: `trait-opacify` (current tip `34916fd73`).

## Previous session deliverables (2026-04-30c)

  - **`Ind_cpa.fst` un-laxed** (`504c881bb`).  Dropped
    `Libcrux_ml_kem.Ind_cpa.fst` from `ADMIT_MODULES` via per-fn
    `verification_status(lax)` cascade.  3 fns at PF/Hacspec end-to-
    end via composition over the lax callees:
    `serialize_public_key`, `serialize_public_key_mut`,
    `generate_keypair` (Q2 5.1 s, full hacspec contract).  16 fns
    per-fn lax.  Module make: 27 s wall (was >800 s cancelled).
    Three legacy rlimits brought to cap (1500/2500/1500 →
    400/400/800).
  - **`Hacspec_ml_kem.Parameters.Sizes` shipped** (`a65ab3e43`).
    Rank-parameterized versions of the top-5 Spec.MLKEM size
    constants (`v_ETA1`, `v_ETA1_RANDOMNESS_SIZE`,
    `v_RANKED_BYTES_PER_RING_ELEMENT`, `v_CPA_PUBLIC_KEY_SIZE`,
    `v_CCA_PRIVATE_KEY_SIZE`) plus equality lemmas to `Spec.MLKEM`.
    Module makes pass in 1.16 s; all 16 queries succeed at rlimit 80.
    Lives at
    `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Parameters.Sizes.fst`.
  - **AP-8 + `feedback_rlimit_cap_800` strengthened** (mid-session
    user re-affirmation).  Never bump rlimit > 800; under
    `--split_queries always` the cap is 400/query; high-rlimit proofs
    are flake debt — fix structurally.  Remediation ladder
    documented (SD4 → factor → split_queries → drive-to-the-top →
    lax).

## DO NOT TOUCH

  - **USER-14** — `invert_ntt_at_layer_4_plus` body
    (`src/invert_ntt.rs:548`).  User is handling.

## Priority order

**1. `Spec.MLKEM` removal pass — per-fn migration in `src/ind_cpa.rs`
(~2 sessions).**  292 Rust hits across `ind_cpa.rs` (147) and
`ind_cca.rs` (145).  Foundation (`Hacspec_ml_kem.Parameters.Sizes`)
already shipped — start using it.  User mandate
`feedback_avoid_spec_mlkem`.

  - **Start with the 3 PF/Hacspec fns in `ind_cpa.rs`**:
    `serialize_public_key`, `serialize_public_key_mut`,
    `generate_keypair`.  These already verify; migrating their
    ensures from `Spec.MLKEM.*` to
    `Hacspec_ml_kem.Parameters.Sizes.*` is low-risk one-shot work.
    For each: edit ensures, re-extract, `make check/Libcrux_ml_kem.Ind_cpa.fst`,
    verify it still passes.  If it fails, bring in
    `lemma_<C>_eq` from `Hacspec_ml_kem.Parameters.Sizes` via a
    `hax_lib::fstar!()` block.
  - **Trace transitive cites BEFORE committing each function.**
    `Libcrux_ml_kem.Vector.to_spec_vector_t` and friends may
    themselves use `Spec.MLKEM` internally.  Grep one hop down
    (e.g. `grep -n "Spec.MLKEM" specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*.fst`)
    before declaring a function migrated.
  - Then move to the lax fns.  Their ensures are still in scope
    (the lax marker only admits the body, not the post).  Migrating
    them establishes the spec contract while leaving the body
    admitted; a future un-lax pass will close the body.
  - **Commit cadence**: one commit per function or small group.
    Prefix `agent-mlkem:`.

**2. Un-lax cascade follow-up on `Ind_cpa.fst` (parallel; ~3-4
sessions).**  16 fns are per-fn lax.  Path-of-least-resistance per
session: pick one cluster (serialize/sample / encrypt/build /
deserialize/decrypt), factor a lemma to module scope where the
per-iteration `eq_intro`'s slice-bound subtyping is heavy
(`feedback_layer2_branch_post_z3_unlock` shape), and un-lax that
function.  Don't bump rlimit — use SD4, factoring, opacity, or
keep lax.

**3. Forward NTT layer 7 bridge (gated, multi-session).**  Lift-
convention design open (see session-2026-04-30b session report
section F4).  Two candidate resolutions:
  (a) Add a `plain_to_mont_lift` transition lemma at the layer-7
      boundary (cheap, contained).
  (b) Re-state forward 1/2/3 ensures under the plain lift
      (`i16_to_spec_array`) so the chain is uniform (expensive,
      cascades through trait bridges).
  Don't start until the design choice is made.

## Anti-patterns (still load-bearing)

  AP-1 Big-axiom-bridge designs (the Serialize.HacspecBridge
       abandonment lesson).  Pick one width, prove it, generalize
       only after the second falls into the same shape.

  AP-2 GLOBAL `reveal_opaque (\`%P) (P)` in loop bodies (Rule SD4)
       — single line caused the 153 s top-1 wall fix dropped to
       2.1 s.  Use targeted form: `reveal_opaque (\`%P) (P arg1 arg2)`.

  AP-3 Bulk-deleting `.checked` files.  Surgically remove only the
       module being iterated.

  AP-4 Trusting hint replay across ensures upgrades.  After every
       milestone closure that strengthens an ensures, re-record
       hints for the module + its consumers and commit them.

  AP-5 (cross-sprint, ml-dsa origin) `bits USIZE` opacity.  Z3
       cannot derive `v x < bits USIZE` from `v x < 64` under
       `fuel 0`; `assert_norm` does NOT unfold it either.  Tighten
       shift bounds rather than detour into proof-libs.

  AP-6 (cross-sprint, ml-dsa origin) `assert_norm` for arithmetic
       constant chains with subtraction.  Plain `assert
       (v $C == K)` fails under `fuel 0` even when Z3 can compute
       the value if the constant chain has a `-` step.

  AP-7 `$<param>` instead of `${<param>}_future` in
  `#[hax_lib::ensures(...)]` for `&mut` parameters.  Renders the
  post on the *input*, not the post-mutation result — usually
  false.  Audit confirmed only one occurrence (fixed
  `bbef9328b`); guard against re-introducing it.

  **AP-8 (REINFORCED 2026-04-30 mid-session)**: bumping
  `--z3rlimit` past 800 — and any rlimit > 400 when
  `--split_queries always` is set.  Prohibited
  (`feedback_rlimit_cap_800`).  Z3 perf at high rlimit is
  non-monotone — proofs flip pass/fail across re-runs.
  **High-rlimit proofs are flake debt.**  When you hit a wall, in
  order: (1) targeted `reveal_opaque (\`%P) (P args)` (Rule SD4);
  (2) factor a lemma to module scope; (3) `--split_queries always`
  + lower per-query rlimit; (4) drive-to-the-top spike;
  (5) accept `verification_status(lax)` and un-lax consumers.
  Never bump.  When TOUCHING legacy code with rlimit > 800
  (or > 400 under split), REDUCE it in the same commit — don't
  preserve the flake.

## Hard rules

  R1 Branch `trait-opacify` directly.  User merges to origin manually.
  R2 No NEW broad admits.  `--admit_smt_queries true` is OK as a
     temporary scaffold; remove before committing each closure.
     Per-fn `verification_status(lax)` is the supported per-function
     admit form.
  R3 No new axioms unless absolutely necessary.  If you must, file
     as SIDEWAYS in `MLKEM_STATUS.md` + commit message.
  R4 `ulimit -v 8388608`.  **F\* `--z3rlimit ≤ 800` HARD CAP**;
     **`--split_queries always` cap is `≤ 400` per query** (memory
     rule `feedback_rlimit_cap_800`).  Default tier:
     `--z3rlimit 200`.  Bump only after profiling.  When touching
     legacy code with rlimit > 800, REDUCE it; don't preserve.
  R5 Inner edit-check: `make check/<Mod>.fst` from
     `proofs/fstar/extraction/`.  Cap iteration at 20 min/attempt.
  R6 Always re-record hints AND touch unchanged `.checked` after
     `python3 hax.py extract` (per `feedback_touch_unchanged_checked`).
  R7 Trait FROZEN — DO NOT touch `src/vector/traits.rs` (the trait
     contract).  Only impl-side bodies, Bridges/Chunk additions, and
     consumer-side strengthening.
  R8 No `fstar-mcp` (per `feedback_use_fstar_mcp` and
     `feedback_fstar_mcp_session_dies_after_make`).
  R9 After each milestone closure: regenerate
     `proofs/ml_kem_verification_status.md` and update `proof_milestones.md`
     status icon.  Commit prefix: `agent-mlkem:`.
  R10 NO new `Spec.MLKEM.*` citations.  Use
  `Hacspec_ml_kem.Parameters.Sizes.*` for size constants
  (`feedback_avoid_spec_mlkem`).  When touching existing
  Spec.MLKEM ensures, opportunistically migrate them in the same
  commit.

## Workflow

  1. Read `proofs/agent-status/session-2026-04-30c.md` (the previous
     session's report — has commit SHAs and findings).
  2. Read `proofs/proof_milestones.md` for current row statuses
     (row 27 is now ⚠️ partial).
  3. Pick the first priority that's actionable.  Make the change.
  4. Iterate `make check/<Mod>.fst` until clean.
  5. `python3 proofs/generate_verification_status.py` to refresh report.
  6. Update `proof_milestones.md` row status + add commit SHA.
  7. Commit with prefix `agent-mlkem:`.  Move to next.
  8. Cap: 4-5 milestones or 4 hours total.

## Per-build hygiene (paste-ready)

  ```bash
  cd ~/libcrux-trait-opacify/libcrux-ml-kem
  find proofs/fstar/extraction -maxdepth 1 \( -name "Libcrux_ml_kem*.fst" -o -name "Libcrux_ml_kem*.fsti" \) | sort | xargs shasum > /tmp/pre.sha
  python3 hax.py extract
  # Local fix for hax codegen bug — duplicate noeq on Neon Vector_type
  sed -i '' '7,8d' proofs/fstar/extraction/Libcrux_ml_kem.Vector.Neon.Vector_type.fsti 2>/dev/null
  find proofs/fstar/extraction -maxdepth 1 \( -name "Libcrux_ml_kem*.fst" -o -name "Libcrux_ml_kem*.fsti" \) | sort | xargs shasum > /tmp/post.sha
  diff /tmp/pre.sha /tmp/post.sha > /tmp/diff.sha
  cd proofs/fstar/extraction
  for f in $(find . -maxdepth 1 \( -name "Libcrux_ml_kem*.fst" -o -name "Libcrux_ml_kem*.fsti" \)); do
    base=$(basename $f); chk=/Users/karthik/libcrux-trait-opacify/.fstar-cache/checked/$base.checked
    if ! grep -qF "$base" /tmp/diff.sha && [ -f "$chk" ]; then touch "$chk"; fi
  done
  ```

  Note: `ulimit -v 8388608` fails silently on macOS (`setrlimit failed:
  invalid argument`).  Skip it locally; the make still runs.

## Migration recipe for priority 1 (per-fn)

For each function in `src/ind_cpa.rs` (start with the 3 verified
ones):

  1. Find every `Spec.MLKEM.<C>` symbol in its `requires`/`ensures`.
  2. Where `<C>` is one of `v_ETA1`, `v_ETA1_RANDOMNESS_SIZE`,
     `v_RANKED_BYTES_PER_RING_ELEMENT`, `v_CPA_PUBLIC_KEY_SIZE`,
     `v_CCA_PRIVATE_KEY_SIZE` (or `v_CPA_PRIVATE_KEY_SIZE` —
     derived), replace with
     `Hacspec_ml_kem.Parameters.Sizes.<C>`.  The post is now stated
     in the new namespace.
  3. If the function body's annotations use the old shape (e.g.
     loop invariants, intermediate asserts), either migrate them
     too OR introduce a one-line `hax_lib::fstar!()` invoking
     `Hacspec_ml_kem.Parameters.Sizes.lemma_<C>_eq <rank>` to
     bridge.  Prefer migrating outright when the cluster is small.
  4. Re-extract.  `make check/Libcrux_ml_kem.Ind_cpa.fst` (~30 s
     wall now).
  5. Commit.  Move to next function.

For symbols NOT in the top-5 (e.g. `v_C1_SIZE`, `v_C2_SIZE`,
`v_CPA_CIPHERTEXT_SIZE`, `is_rank`, `vector_encode_12`,
`byte_encode`, `ind_cpa_encrypt`, etc.) — DO NOT migrate yet.
Either skip the function or extend `Parameters.Sizes` with the
missing constant first (one commit, then per-fn migration).
`is_rank` and `rank` are already exported from
`Hacspec_ml_kem.Parameters.Sizes` and can replace `Spec.MLKEM.is_rank`.

## Deliverable

End-of-session report (≤ 200 words) at
`proofs/agent-status/session-<date>.md`:
  - Milestones closed (✅) with commit SHAs.
  - Milestones partially advanced (🔶 → improved).
  - Any new admits/axioms (R3) or new lax markers (R2).
  - F\* perf delta on affected modules vs
    `proofs/agent-status/fstar-perf-top20.md`.
  - Final commit SHA.
  - Spec.MLKEM hit count delta (run
    `grep -rn "Spec\.MLKEM" specs/ml-kem/src libcrux-ml-kem/src | wc -l`
    before and after).
  - Next-priority recommendation for the FOLLOWING session.

DO NOT push to origin.  DO NOT touch `~/libcrux-ml-dsa-proofs` or
`~/libcrux-sha3-focused`.  DO NOT touch `invert_ntt_at_layer_4_plus`
(USER-14, user-handled).

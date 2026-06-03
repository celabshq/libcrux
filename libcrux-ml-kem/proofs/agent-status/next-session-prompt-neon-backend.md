# Session prompt — ML-KEM Neon backend verification (Sprint 1 of ~3)

## Goal

Bring the Neon (aarch64/simd128) backend of libcrux-ml-kem out of
ADMIT_MODULES, mirroring the now-complete AVX2 backend (0 lax as of
6e7cb0566) and reusing the Arm64 intrinsics-modeling patterns that took
libcrux-sha3's Arm64 backend to zero admits
(`~/libcrux-sha3-proofs`, commit 75721cd1f-era work).

Scoreboard at start: Neon = 7 modules / 83 fns / **82 lax + 1 unverified**
(the only remaining lax bucket besides 5 Generic fns; Portable and AVX2 are
both 0-lax / 100% panic-safe). Whole-crate Lax 87 (9.1%).

## Current state (verified facts, do not re-derive)

- `libcrux-ml-kem/src/vector/neon/` = arithmetic, compress, ntt, serialize,
  vector_type (.rs, 793 lines total) + `sampling.rs` (NOT compiled —
  `// mod sampling;` commented out in `neon.rs`; the "1 unverified" fn).
- `src/vector/neon.rs`: `#[hax_lib::attributes] impl Operations for
  SIMD128Vector` already delegates every method to free fns in the
  submodules (structurally flattened), but only ZERO/from_i16_array/
  to_i16_array carry posts. The backend was admitted wholesale when the
  trait posts were strengthened (see Makefile "Trait-boundary admits"
  comments) and never retrofitted.
- `vector_type.rs`: `SIMD128Vector { low, high: _int16x8_t }` with
  `noeq` + `val repr ...` + **`let repr (x) = admit()`** — an admitted
  definition to eliminate early (define via the lane views, sha3-style).
- ADMIT_MODULES (extraction Makefile lines 5-12): Vector.Neon.{Arithmetic,
  Compress,Ntt,Serialize,Vector_type}.fst, Vector.Neon.{fst,fsti},
  Vector.Neon.Vector_type.fsti. The .fsti entries mask **Error 162**
  (decidable-equality module-level issue, Phase 6d directive) — first
  blocker to actually diagnose: build the un-admitted .fsti and read the
  error. sha3's Arm64 modules (same `bit_vec 128` payload types) verify, so
  a known-good treatment exists — compare their type/interface setup.
- Intrinsics models: `crates/utils/intrinsics/src/arm64_extract.rs`
  (94 fns) ALREADY has per-lane ensures in the same style as the AVX2
  models (e.g. `_vaddq_s16`: `forall (i:nat{i<8}). get_lane_i16x8 $result i
  == get_lane_i16x8 $lhs i +. get_lane_i16x8 $rhs i`). The sha3 repo's copy
  (`~/libcrux-sha3-proofs/crates/utils/intrinsics/src/arm64_extract.rs`)
  is AHEAD: it adds (a) `.fst`-side `assume val vecN_as_TxM_axiom` lane-view
  definitions, (b) richer byte/bit-level posts on loads/stores
  (`_vst1q_bytes_u64` with `get_bit`-level posts). Diff the two files and
  port the sha3 improvements into THIS repo's copy (Rust source only;
  re-extract; never hand-edit extracted F*).

## Assets / recipes to reuse

1. **AVX2 backend as the blueprint** (`src/vector/avx2.rs` + submodules):
   - per-method trait contracts on the impl (bare `#[requires]/#[ensures]`
     under `#[hax_lib::attributes]`, citing `impl.f_repr`,
     `Libcrux_ml_kem.Vector.Traits.Spec.*` predicates);
   - `op_serialize_N_{pre,post}_bridge` lemma pairs in a `fstar::before`
     block bridging BitVec primitive posts to `serialize_{pre,post}_N`
     (all five N now proven — op_serialize_1 landed 6e7cb0566; copy the
     N=4 pair as template, plus `bit_vec_of_int_t_array_*_lemma`
     decomposition + `BitVecEq.bit_vec_equal_intro`). Neon will need the
     vec128 analogues of the decomposition lemmas — check what sha3
     already provides on `_int16x8_t`/`_uint8x16_t`.
   - branch-post opacity for nested-if ladders (compress/cond_subtract):
     4 per-branch helpers with concrete bound + per-lane wrapper +
     `--split_queries always` on the composition (memory: Layer-2
     branch_post Z3 unlock).
   - ground-literal SIMD proofs: per-lane index-parameterized facts,
     ground conjunctions over 8/16 lanes; avoid vector-level ITE-foralls;
     thread masks as free params (BitVec closures lack congruence).
2. **Portable backend** (`src/vector/portable.rs`): the exact trait
   pre/post texts per method — Neon's contracts should be copy-adjusted
   from there (the spec is backend-independent).
3. **sha3 Arm64 patterns** (`~/libcrux-sha3-proofs`):
   - lane-view modeling (`vec128_as_i16x8` + `get_lane_i16x8` +
     `unfold type _int16x8_t = bit_vec 128`);
   - opaque-predicate store/load recipe (modifies_range/stored atoms,
     top-down composition, free-vec leaf producers — memory:
     feedback_opaque_predicate_store_proof);
   - KNOWN CLIFF: `Libcrux_sha3.Simd.Arm64.load_block` query 301
     deterministically times out at rlimit 800 — if a Neon ml-kem
     load/store proof hits the same shape, don't grind it; restructure
     per the sha3 store_block recipe instead.
   - sha3 judged costs acceptable at leaves; keep high-level fns automatic
     (memory: feedback_high_level_automatic_leaf_cost).

## Plan (this session = Stages 0-3; serialize/NTT/compress are Sprints 2-3)

- **Stage 0 — recon + baseline**: confirm 6e7cb0566-era tree, run
  `cargo test --features simd128,mlkem512,mlkem768,mlkem1024 --test self`
  (THIS HOST IS ARM64 — neon code runs natively; cargo test is the inner
  loop, F* batched). Read MLKEM_STATUS.md Phase 6d notes on Error 162.
- **Stage 1 — intrinsics sync**: port sha3's arm64_extract.rs deltas into
  crates/utils/intrinsics (lane-view .fst axioms, load/store bit-level
  posts). cargo test per edit; one extraction at the end of the stage.
  NOTE: extraction covers crates/utils/intrinsics too (its own stanza in
  hax.py) — re-verify Libcrux_intrinsics.* after.
- **Stage 2 — Error 162 + vector_type**: un-admit
  Vector.Neon.Vector_type.{fst,fsti}; diagnose Error 162 with the real
  error in hand (compare sha3's noeq/interface treatment); replace
  `let repr = admit()` with a real definition via the lane views; add
  to_i16_array/from_i16_array/to_bytes/from_bytes contracts (mirror
  portable + sha3 store/load posts). Per-stage clean rebuild: rm the
  touched modules' .checked, re-make, real Query-stats required.
- **Stage 3 — arithmetic.rs**: per-lane contracts on add/sub/
  multiply_by_constant/bitwise_and_with_constant/shift_right/
  cond_subtract_3329/barrett_reduce/montgomery_multiply* mirroring
  portable.rs's op_* specs; un-admit Vector.Neon.Arithmetic.fst. The
  intrinsics' per-lane ensures (`+.` etc. are wrapping machine ops) feed
  these directly; mind `%!` Euclidean-vs-Rust-mod (memory) in any
  mod-q specs, and keep SMTPats per the lane-propagation policy (per-lane
  equality + one reveal at this layer; bridge at the trait boundary).
- **Defer**: serialize.rs (needs the vec128 bit_vec bridge lemmas — its
  own sprint), ntt.rs + compress.rs (branch_post opacity sprint),
  `mod sampling` re-enable (CODE change — needs Karthik's explicit OK),
  and the final impl-Operations contract pass + ADMIT_MODULES removal.

## Constraints (standing mandates — non-negotiable)

- NO runtime code changes (annotations/proof scaffolding only); preserve
  code, adapt proofs, modify spec. `mod sampling` stays commented out
  unless Karthik approves.
- Max 4 concurrent fstar.exe+z3 (make -j2). Never pkill by name. Never
  bulk-delete or mtime-bump .checked (targeted rm of touched modules at
  stage boundaries is the sanctioned per-stage clean rebuild).
- z3rlimit cap 800 (400 with split_queries). smtprofiling before ANY
  cliff/blocker claim. 30-60 min per-function debug budget, then mark
  user-followup and move on.
- Never hand-edit extracted F*; all changes in Rust + re-extract
  (`python3 hax.py extract` in libcrux-ml-kem, ~2.5 min). cargo-hax 0.3.7
  (opam switch hax-0.3.7).
- Ensures cite Hacspec_ml_kem.* / Traits.Spec.*, never Spec.MLKEM.
- fstar MCP proxy was flaky on 2026-06-03; direct
  `make -j2 check/<Module> > /tmp/log 2>&1` is the accepted fallback —
  judge by exit 0 + real Query-stats lines (no-stats <1s pass = stale
  .checked). fstar-mcp JSONL timestamps are UTC; host clock CEST.
- hax fstar! antiquote gotchas: space before `}` after `$X`;
  `${ident}.field` braced; bare `fstar!` inside `loop_invariant!`;
  machine-product WF in invariants → leading `i <= K`-style bound conjunct.
- Trait impls take bare `#[requires]/#[ensures]` under
  `#[hax_lib::attributes]` (the old E0438 lore is wrong);
  `verification_status(lax)` does NOT work on trait-impl methods (body
  `fstar!("admit ()")` if ever needed).
- If running as a spawned agent: status report every 15 min to
  proofs/agent-status/agent-neon-sprint1-status.md (sub-task, blocker,
  ETA); final report with build IDs, query stats, files changed.

## Exit criteria for Sprint 1

- Intrinsics model synced + Libcrux_intrinsics re-verified.
- Vector.Neon.Vector_type.{fst,fsti} + Vector.Neon.Arithmetic.fst out of
  ADMIT_MODULES, verified with real query stats; `repr` admit gone.
- Full-crate `make all` gate green; cargo simd128 tests green.
- Status doc regenerated (expect Neon lax 82 → ~60); perf top-20 refreshed;
  next-sprint prompt written (serialize bridges = Sprint 2, ntt+compress +
  final impl pass + ADMIT_MODULES removal = Sprint 3); commit per
  `agent-mlkem:` convention (no push).

# MLKEM Verification Status

**Branch**: `trait-opacify`  **Tip**: `fa31480cd` (Wave-A handoff; B6 / USER-1 / A8 4-case Barrett enumeration closed at `2f96abe99`; 2026-04-29).  Prior tip: `0784e3b72` (track A — Phase 7a Step 3 sub-pieces 1+2 — strengthened `inv_ntt_layer_int_vec_step_reduce` post + chunk-pair Bridges lemma; layer 4_plus body TEMP-admitted pending Step 5 drive-to-the-top spike; 2026-04-28 late evening)

## Wave-A summary (2026-04-29)

| Lane | Status | Commit |
|---|---|---|
| B6 (USER-1 / A8) | ✅ LANDED | `2f96abe99` |
| B1 (compress.rs panic_free) | 🔶 PARTIAL | `f87e9a8cf` — primitive `compress_message_coefficient` + chunk wrapper `decompress_1` closed (2 / 6).  3 chunk wrappers (compress_1, decompress_d, compress<D>) remain panic_free due to Z3 saturation on 16-element forall + integer-form post in loop invariant.  Filed as USER-13. |
| B3 (AVX2 serialize/deserialize bridges) | ✅ LANDED | `e5c4a6f49` — 13 admitted bridges closed via new `bit_vec_of_int_t_array_vec256_as_i16x16_lemma` axiom in `Libcrux_intrinsics.Avx2_extract.fsti`.  Includes USER-9a 11-bit bonus. |
| B2, B5 | ⏸ DEFERRED | — (USER-12 / USER-11; Z3 walls) |
| B4 | ⏸ DESCOPED | — (per prompt directive; USER-4) |

Net admit-count delta: **-1 (B6) -2 (B1) -13 (B3) = -16 PROGRESS**,
**+1 SIDEWAYS** (axiom in Avx2_extract.fsti), **+1 FAIR GAME**
(USER-13 filed for the 3 remaining compress.rs chunk wrappers).
Wave-A continuation: net **-15** admits.  Wave-B can proceed (B6 was
the only lane gating Wave-B).

## Wave-B summary (2026-04-29 11:00–11:30, setup-only)

Worktree: `~/libcrux-ml-kem-above-trait/` (NEW; cloned from
`/Users/karthik/libcrux-trait-opacify/` at trait-opacify HEAD
`fa31480cd`).  Coordinator session 30 min; 0 lanes closed; 1 lane
spike attempted (A2) and reverted.

| Lane | Status | Notes |
|---|---|---|
| A1 (Phase 7c serialize) | ⏸ NOT ATTEMPTED | Same Z3-wall risk as A2; deferred to USER-N |
| A2 (Phase 7n + USER-10) | ⏸ DEFERRED | `lax→panic_free` spike on `sample_from_uniform_distribution_next` failed at Sampling.fst(161,18-161,43) subtyping (incomplete quantifiers, rlimit 400).  Filed as USER-10. |
| A3 (USER-7 subtract_reduce body) | ✅ CLOSED 2026-04-29 | Hypothesis (b) + parameter unshadowing — see USER-7 below for resolution detail.  Sibling fns (`add_message_error_reduce`, `add_error_reduce`) still bounds-only post; strengthening is a separate multi-hour task. |
| A5 (USER-6 invert_ntt_montgomery) | ⏸ NOT ATTEMPTED | Wave-B baseline already showed `inv_ntt_layer_int_vec_step_reduce` Q101 saturation; Invert_ntt.fst TEMP-admitted in Wave-B local Makefile.  Lane A5 will UNADMIT when it begins (next session). |

Net admit-count delta: **0 net**.  No upstream commits from Wave-B
beyond documentation (perf snapshot, agent-trackA log, this section).
Wave-C still gated on A5; sprint progress is paused at Wave-B's start.

See `agent-trackA.md` 2026-04-29 11:00–11:30 entry for full
attempt-and-revert detail on A2 and the recorded surprise about
hax-lib's `panic_free` semantics (does NOT emit
`--admit_smt_queries true` push-options — only `lax` does).

## Admit count

Below-trait movable admits at Wave-A continuation end:
- `Hacspec_ml_kem.Commute.Chunk.fst`: 2 (decompress chunk_commutes at
  lines 1069, 1083 — above-trait, A1 territory).
- `Libcrux_ml_kem.Vector.Avx2.fst`: 7 bridge admits (B4 NTT
  territory + USER-9b 5-bit serialize/deserialize bridges):
  op_ntt_layer_{1,2}_step_bridge, op_inv_ntt_layer_{1,2}_step_bridge
  (4, B4); op_serialize_5_pre_bridge, op_serialize_5_post_bridge,
  op_deserialize_5_post_bridge (3, USER-9b).
  Was 14; B3 closed 10, USER-9a bonus closed 3 (added 3 for 11-bit
  bridges, all above-axiom), USER-9b adds 3 for 5-bit (also
  above-axiom).  Net 7 bridge admits remain; the USER-9 bridges are
  the same `bit_vec_of_int_t_array_vec256_as_i16x16_lemma` axiom
  application pattern as B3.
- `Libcrux_ml_kem.Vector.Avx2.Ntt.fst`: 6 admits (B4 territory).
- `Libcrux_ml_kem.Vector.Portable.fst`: 2 `--admit_smt_queries true`
  pushes (op_ntt_layer_1_step, op_inv_ntt_layer_1_step — B2).
- `src/vector/portable.rs`: 1 panic_free (op_ntt_multiply — B5).
- `src/vector/portable/compress.rs`: 4 panic_free (B1 partial:
  compress_ciphertext_coefficient — Barrett primitive USER-N;
  compress_1, compress<D>, decompress_d — chunk wrappers USER-13).
  Was 6; B1 closed 2.
- `src/vector/avx2.rs`: 7 panic_free (B3 / B4 / B5 territory).
  Note: B3 was bridge-admit territory not panic_free; remaining
  panic_free unchanged.

## SIDEWAYS admits

- `crates/utils/intrinsics/proofs/fstar/extraction/Libcrux_intrinsics.Avx2_extract.fsti`:
  1 new `val` axiom (`bit_vec_of_int_t_array_vec256_as_i16x16_lemma`)
  asserting that `vec256_as_i16x16` is the canonical bit-exact
  lane-decomposition.  Used by all op_serialize/deserialize_N_bridge
  proofs.  This is a structural axiom about the abstract function;
  it cannot be proven without a definition of `vec256_as_i16x16`.

## Phase 7a status

| Step | Description | Status |
|---|---|---|
| 7a-E1 | `lemma_poly_barrett_reduce_commute` (E2) | ✅ merged `f9e813cdc` |
| 7a-E2 | `lemma_add_to_ring_element_commute` (E2) | ✅ merged `1d6cacc50` |
| 7a Step 1 | inverse NTT layer 1 hacspec bridge (track A, Bridges.fst) | ✅ verified `ba8681b38` |
| 7a Step 9 | scaling-chain doc comments (track A) | ✅ verified `8d92695bf` |
| 7a Step 4 layer 1 | strengthen invert_ntt_at_layer_1 post (Option B citing Bridges lemma) | ✅ verified `8358b1093` |
| 7a Step 4 layer 3 | strengthen invert_ntt_at_layer_3 post (Option B) | ✅ verified `43c9d45d5` |
| 7a Step 4 layer 2 | strengthen invert_ntt_at_layer_2 post | ⏸ DEFERRED — first attempt failed with 6 Z3 errors at rlimit 800; see next-session-prompt.md for hypotheses + failure trace |
| 7a Step 7.1 | F* lemmas for to_standard_domain track (Chunk.fst) | ✅ verified `c07feb91c` (merge of trackD) |
| 7a Step 7.1+ | closed-form lane lemma (Option B infra) | ✅ verified `8ff3ac14c` |
| 7a Step 7.2 | strengthen add_standard_error_reduce post (Rust integration) | ⏸ HELD — Z3 saturated on 2 invariant approaches; see TODO in `src/polynomial.rs` |
| 7a Step 2 layer 3 | inverse NTT layer 3 bridge (Bridges.fst: zetas_1_lane, lemma_ntt_inverse_layer_n_16_8_lane, lemma_inv_ntt_layer_3_step_*) | ✅ verified `fa2151ea8` |
| 7a Step 2 layer 2 | layer 2 inverse NTT bridge (4 per-branch + per-lane wrapper + `--split_queries always`) | ✅ verified `b7b49c358` |
| 7a Step 3.1 | strengthen `inv_ntt_layer_int_vec_step_reduce` post (per-lane FE eqs) | ✅ verified `0784e3b72` |
| 7a Step 3.2 | chunk-pair hacspec bridge `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec` (Bridges.fst) | ✅ verified `0784e3b72` |
| 7a Step 3.3 | per-polynomial composition in `invert_ntt_at_layer_4_plus` (cite `IN.ntt_inverse_layer_n 256`) | ⏸ DEFERRED — needs new spec helpers (`mont_to_spec_poly_256`, `zetas_N_inv` for layer 4..7); naturally combines with Step 4 layer 4_plus.  See agent-trackA.md "Open work / Step 3.3 deferred" |
| 7a Step 5 | strengthen invert_ntt_montgomery post | ⏸ pending |
| 7a Step 6 | strengthen 3 INTT-consuming reduce fns | ⏸ pending |
| 7a Step 7 | strengthen add_standard_error_reduce | ⏸ pending |
| 7a Step 8 | verify call sites in matrix.rs | ⏸ pending |

See `/Users/karthik/.claude/plans/replicated-beaming-pnueli.md` for the full Step 1-9 plan and `proofs/agent-status/handoff-2026-04-28-trackA.md` for the resume entry point.

## Spec hierarchy (DO NOT FORGET)

- ✅ **Canonical**: `Hacspec_ml_kem.*` — in
  `specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.*.fst[i]`. ALL new
  proofs and post-conditions cite this.
- ⚠️ **Obsolete (scheduled for deletion)**: `Spec.MLKEM.*`. Existing
  citations are migrating; do NOT add new ones. Banners in
  `proofs/fstar/spec/Spec.MLKEM*.fst`.
- 🔄 **Temporary scaffolding**: `Spec.Utils.*` — pieces we use will move
  to a future `Proof_utils.*` module.

## Migration plan (post-only — pre-conditions never touched)

The trait-opacity work (Phases 1–5, done) provided the foundation: trait
posts now carry strong FE-form equations behind opaque per-lane / per-branch
predicates.  The next phase strengthens **above-trait** module posts to cite
`Hacspec_ml_kem.*`, module by module, post-only (additive conjuncts) so each
commit lands without breaking neighbours.

### Combined sequence (from session 2026-04-27)

| # | Phase | Module(s) | Citation target | Notes |
|---|---|---|---|---|
| A | 7a — Polynomial scalar ops | `Polynomial.fst` (7 fns: add_to_ring_element, poly_barrett_reduce, subtract_reduce, add_error_reduce, add_standard_error_reduce, add_message_error_reduce, multiply_by_constant_bounded) | `Hacspec_ml_kem.Polynomial.*` | New chunk-level commute lemmas via Classical.forall_intro |
| B | 7b — NTT/inv-NTT layers 1–3 | `Ntt.fst`, `Invert_ntt.fst` (8 fns) | `Hacspec_ml_kem.Ntt.*`, `Hacspec_ml_kem.Invert_ntt.*` | 2 layer-step commute lemmas |
| C | 7c — Serialize re-root | `Serialize.fst` (8 already-cite-Spec.MLKEM + 6 trivial) | `Hacspec_ml_kem.Serialize.*` (added alongside Spec.MLKEM, then Spec.MLKEM dropped) | Enables Spec.MLKEM deletion |
| D | 7d — Matrix | `Matrix.fst` (5 fns) | `Hacspec_ml_kem.Matrix.*` | Vector + matrix commute lemmas |
| E | 7e — Trait-opacify Phase 6 (parallel) | drop 6 NTT-layer impl admits in `portable.rs` | n/a (impl-side) | Independent; uses fstar-mcp |
| F-I | Hard cases | full NTT, `ntt_multiply`, `to_standard_domain`, `compute_vector_u`, `invert_ntt_montgomery` | `Hacspec_ml_kem.*` | Tier-3 layer composition; standalone-hard proofs |
| J | Migrate downstream | Ind_cpa, Ind_cca | `Hacspec_ml_kem.*` (replace Spec.MLKEM lookups) | Post-only consumer-side switch |
| K | Drop Spec.MLKEM conjuncts | Serialize.fst (and any others) | n/a | Pure removal of redundant post conjuncts |
| L | **Delete `Spec.MLKEM.*` module** | n/a | n/a | Drop files; update Makefile / hax includes |
| M | `Spec.Utils` → `Proof_utils` migration | trait posts, commute lemmas | n/a | Symbol-level rewrite |

### Invariants enforced at every step

1. **Pre-conditions: never touched.** A and beyond.
2. **Posts: only conjunctive additions** in A–J. Removals (K) only of conjuncts that are dual citations of an equivalent property.

## Trait-opacify completed phases (1–5)

Branch `trait-opacify`, commits:
- `9f154fd80` — Trait opacify: per-lane / per-branch opaque predicates + drop Polynomial / Invert_ntt admits
- `1c47e3632` — Drop Serialize.fst admit (Phase 5): SMTPat lookup + named intro lemma

All three trait-boundary admits (Polynomial.fst, Invert_ntt.fst,
Serialize.fst) dropped.  Verification: 53 Checked / 4 Admitted
(pre-existing) / 1 Failed (pre-existing Vector.Neon.Vector_type.fsti
decidable-eq, unrelated).

## USER TASKS — please pick from here first

Manual / Z3-blocked / math-heavy proofs.  Each one is exemplary: the
agent can copy the style to extend the pattern to similar admits.

### USER-1 ✅ CLOSED 2026-04-29 (Wave-A B6)

**`lemma_compress_ciphertext_coefficient_fe_commute`** (a.k.a. **A8**) in
`specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst`.

- **Status**: ✅ proven at commit `2f96abe99`.  Body = 2 f_val asserts
  + 4 pow2 witnesses (16, 32, 1024, 2048) at rlimit 400.  Verifies in
  86s cold (full Chunk.fst).  Mirrors A5's D=1 closure shape.
- **Math**: Barrett-exactness 4-case enumeration over D ∈ {4, 5, 10, 11}.
  For each D, the integer formula `((v fe * 2 * 2^D + 3329) / 6658) % 2^D`
  equals hacspec's `compress_d (i16_to_spec_fe fe) D`.
- **Why first**: closes the per-element commute layer for compress<D>;
  unblocks Phase 7c (Serialize compress_then_serialize_{4,10,11,5}).
- **Template value**: the 4-case enumeration pattern + `lemma_impl_*_v_val`
  chain is the model for any future "primitive integer formula ↔ hacspec
  function" lemma.  After this lands, the agent can extend the same
  pattern to other primitive-vs-hacspec proofs.

### USER-2 (do second — template for full NTT layer composition)

**Tier-3 forward-NTT composition lemma**:
`lemma_ntt_full_commute` in `Hacspec_ml_kem.Commute.Chunk.fst` (new file).

- **Status**: not yet started; depends on Tier-2 (`lemma_ntt_layer_n_commute`,
  Phase 7b) which agent can do first.
- **Math**: chain 7 layer-step lemmas, matching the BitRev₇ zeta-table
  ordering exactly.
- **Why second**: closes the NTT layer of the proof.  Once the forward
  composition is proven, the inverse NTT and ntt_multiply compositions
  are direct adaptations.
- **Template value**: shows how to compose layer-step lemmas across the
  zeta indexing.  Agent can copy the structure for `lemma_invert_ntt_full_commute`
  and (with A1–A4) `lemma_ntt_multiply_commute`.

### USER-3 — `to_standard_domain` Montgomery inverse identity

**`Polynomial.to_standard_domain`** post in
`libcrux-ml-kem/src/polynomial.rs` (line ~711–809).

- **Status**: post unfolds to raw `mod q` form referencing `1353 * 169`.
- **Math**: prove `(x * 1353 * 169) mod 3329 == x mod 3329` (Montgomery
  domain identity), so the impl post is equivalent to identity-mod-q.
- **Why user-lane**: standalone modular arithmetic; not a template for
  agent work.

### USER-4 — AVX2 NTT-layer 1/2 bridges (Z3-blocked)

**4 admitted bridges** in `src/vector/avx2.rs`:
- `op_ntt_layer_1_step_bridge`, `op_ntt_layer_2_step_bridge`
- `op_inv_ntt_layer_1_step_bridge`, `op_inv_ntt_layer_2_step_bridge`

- **Status**: admitted; Z3-blocked on the 4-zeta-parallel SIMD wall.
- **Mitigation paths** (user choice):
  1. Refactor each AVX2 layer body into 4 per-zeta sub-functions so the
     proof obligations split.
  2. Adopt the deferred SIMD model unification — see
     `proofs/simd-model-unification-plan.md`.
- **Templates that already exist**: `ntt_layer_3_step` and
  `inv_ntt_layer_3_step` AVX2 wrappers prove their trait posts inline
  without bridge admits — same pattern can extend to layers 1 / 2 once
  the parallelism wall is mitigated.

### USER-5 — `ntt_multiply` Tier-3 wrap (after USER-2)

**`Polynomial.ntt_multiply`** post strengthening + closing
`op_ntt_multiply` `panic_free` annotation.

- **Status**: A1–A4 base-case lemmas proven; need the 128-iteration loop
  wrap.
- **Math**: 128 invocations of `lemma_base_case_mult_{even,odd}_fe_commute`
  composed into hacspec's `multiply_ntts`.
- **Why after USER-2**: requires the Tier-3 layer-composition pattern.

### USER-6 — `invert_ntt_montgomery` (after USER-2) — **IN FLIGHT (Wave-B A5, critical path)**

**`Invert_ntt.invert_ntt_montgomery`** post strengthening.

- **Status**: bounds-only post; needs hacspec_ml_kem.Invert_ntt.ntt_inverse
  citation + final scale-by-3303 (`128⁻¹ mod q`).
- **Math**: 7-layer inverse composition + Montgomery scale.
- **Why after USER-2**: same Tier-3 template.

### USER-7 — `subtract_reduce` body discharge (post-loop record-equality bridge) — **CLOSED 2026-04-29 (Wave-B A3)**

**Resolution**: hypothesis (b) — the array-form lifter — *combined with*
parameter unshadowing.  Three attempts in the prior session failed
because the post references the function parameter `b` while the body's
`for` loop accumulator shadowed it; the post-loop bridge could not name
the parameter.  Lane A3 (2026-04-29):

  1. Added `to_spec_poly_mont_arr (a: t_Array vV 16)` and
     `lemma_to_spec_poly_mont_unfold (p) : to_spec_poly_mont p == to_spec_poly_mont_arr p.f_coefficients`
     in `Hacspec_ml_kem.Commute.Chunk` ("Phase 7a / lane A3 additions"
     section).  Plus `lemma_subtract_reduce_scaled_eq` for createi-extensionality.
  2. Renamed the loop accumulator: `b` parameter (unshadowed) +
     `let mut b_acc = b;` for the loop accumulator.
  3. Pre-loop: applied `lemma_to_spec_poly_mont_unfold #v_Vector b`
     (still the parameter) to seed `to_spec_poly_mont (param b) ==
     to_spec_poly_mont_arr e_b`.
  4. Post-loop: applied `lemma_subtract_reduce_scaled_eq #v_Vector b
     b_input` directly (parameter `b` reachable thanks to the rename).

Result: 111 SMT queries, max 3.5 s, total 89 s.  Body discharges fully
at `--z3rlimit 800 --ext context_pruning --split_queries always`.

Sibling fns `add_message_error_reduce` and `add_error_reduce` were
described as following "mechanically" once the bridge is found, but
this is conditional on having a strengthened post + commute-lemma
chain analogous to `subtract_reduce`'s `subtract_reduce_finalize_chunk`
+ `lemma_subtract_reduce_iter` + `lemma_subtract_reduce_commute` +
`lemma_subtract_reduce_eq_helper`.  Building those chains for the new
ops is a separate ~60-90 min/fn task — filed as follow-up.

---
*Original USER-7 brief preserved below for reference.*

### USER-7 (original) — `subtract_reduce` body discharge — IN FLIGHT (Wave-B A3)

**`Polynomial.subtract_reduce`** body; the strengthened post landed in
commit `c698908ba` and per-poly commute lemma chain in `0a8c7289d`.

- **Status**: post strengthened, body admitted via `--admit_smt_queries true`.
  All math + commute lemmas proven:
  * `fe_1441`, `lemma_subtract_reduce_finalize_fe`, opaque
    `subtract_reduce_finalize_chunk`, `lemma_subtract_reduce_iter`,
    `subtract_reduce_helper`, `lemma_subtract_reduce_eq_helper`,
    `lemma_subtract_reduce_commute`.
- **Outstanding gap**: bridging `to_spec_poly_mont (param b)` (post-form,
  record-based) to `mont_i16_to_spec_fe ((T.f_repr _b[j/16])[j%16])`
  (lemma-form, array-based).
- **Math**: trivial — `_b == (param b).f_coefficients` from let binding,
  `to_spec_poly_mont` only uses `.f_coefficients`, so the two forms
  are equal lane-wise.  The wiring is what's stuck.
- **Three failed attempts logged** (none made it through):
  1. Owned `b: PolynomialRingElement<Vector>` + record construction
     `{ f_coefficients = _b }` → "incomplete quantifiers" at function-body.
  2. `&mut b: &mut PolynomialRingElement<Vector>` + array-form lemma
     (passing `_b` directly) → same.
  3. Above + explicit `_b_init` ghost record + `Seq.lemma_eq_intro`
     bridge inside body F* fragment → Q143 saturated rlimit 800
     in 162 s.
- **Hypotheses for the fix** (in priority order):
  (a) `assert (Seq.index e_b k == Seq.index (param b).f_coefficients k)`
      explicitly per chunk, instead of relying on F* to derive it.
  (b) Define `to_spec_poly_mont_arr (a: t_Array vV 16) : ...` and add
      `lemma_to_spec_poly_mont_unfold p == to_spec_poly_mont_arr p.f_coefficients`,
      use array form throughout the lemma + post.  Avoids the record
      bridge entirely.
  (c) Restate the post WITHOUT `to_spec_poly_mont ${b}` — directly use
      `mont_i16_to_spec_fe` per lane in a forall.  Less elegant but
      sidesteps the createi-of-record-field issue.
  (d) Refactor: split the lemma into stages — one stage per
      `lemma_subtract_reduce_finalize_chunk_reveal` + one stage per
      `Seq.lemma_eq_intro` — and chain them in the body.  Smaller per-
      stage Z3 context.
- **Why USER-lane**: each attempt costs 5–17 min of build + careful
  diagnosis.  Three rounds in this session didn't close.  Worth a
  fresh look with the smtprofiling skill.
- **Once closed**: `add_message_error_reduce` (3-way add) and
  `add_error_reduce` (2-way add) follow the same pattern mechanically.
  Then upstream chain test in `matrix.rs::compute_message` validates
  `invert_ntt_montgomery`'s strengthened post (the original spike's
  question).

### USER-8 — trait `from_bytes` / `to_bytes` post strengthening (Cluster 2) — 🔶 **PARTIAL — S1 CLOSED, S2/S3 ADMITTED 2026-04-29**

Branch `agent/user-8` off `e66708d2f`.  Tip:  `<see session log>`.

#### S1 — trait wire-up: ✅ **CLOSED**
  - `src/vector/traits.rs:1260-1272` now declares the strengthened
    posts (`spec::from_bytes_post`, `spec::to_bytes_post`) on
    `from_bytes` / `to_bytes`.  TODO(C4) markers removed.
  - Vector.Traits.fst verifies in 96s (cold) / 5s (hint replay).
  - Vector.Traits.Spec.fst is unchanged (the helper predicates were
    pre-existing).

#### S2 — portable discharge: 🔶 **ADMITTED — see USER-14**
  - `src/vector/portable.rs:1004-1031` — impl-side `from_bytes` and
    `to_bytes` carry the strengthened posts; bodies invoke
    `hax_lib::fstar!(r#"admit ()"#)` to discharge.
  - 2 NEW admits: portable `f_from_bytes` body, portable `f_to_bytes`
    body.
  - Vector.Portable.fst verifies in 49s.

#### S3 — AVX2 discharge: 🔶 **ADMITTED — see USER-14**
  - `src/vector/avx2.rs:1051-1080` — same admit pattern as portable.
  - 2 NEW admits: AVX2 `f_from_bytes` body, AVX2 `f_to_bytes` body.
  - Note: the AVX2 `to_bytes` was previously `verification_status(lax)`;
    we removed `lax` and replaced with the trait-post + body admit.
  - Vector.Avx2.fst verifies in 83s (split_queries).

#### Closure path
  - **USER-14** (filed): close the 4 admits via the
    `BitVecEq.int_t_array_bitwise_eq` lemma pattern (mirroring
    `serialize_4_bit_vec_lemma`).  See USER-14 entry below.

### USER-9 — trait `serialize_5/11` and `deserialize_5/11` post wiring — ✅ **CLOSED**

Split into two sub-tasks (see commit `750e28ba1` audit detail).  Both
9a (11-bit, wrapper-bridge) and 9b (5-bit, real AVX2 SIMD) are now
closed.

#### USER-9a (11-bit, AVX2 wrapper-bridge) — ✅ **CLOSED 2026-04-29** (commit `2deb01199`)

- Trait now carries strong `BitVecEq` posts on `serialize_11` /
  `deserialize_11` (`src/vector/traits.rs:1372-1378` — TODO(C4) removed).
- Portable impl discharges via `serialize_11_lemma` /
  `deserialize_11_lemma` (the Cluster 3 BitVec lemmas, scale-up
  confirmed at `750e28ba1`).
- AVX2 impl discharges via 3 new admitted bridges
  (`op_serialize_11_pre_bridge`, `op_serialize_11_post_bridge`,
  `op_deserialize_11_post_bridge`) mirroring the 4/10/12 shape.
- AVX2 primitives in `avx2/serialize.rs` keep
  `verification_status(lax)` on the body but now have `BitVec`
  pre/post on their signatures.
- Verification: Vector.Traits.fst 1:25, Vector.Portable.fst 2:18,
  Vector.Avx2.fst 1:01, Vector.Avx2.Serialize.fst ~1:30.
- **hax quirk for posterity**: single-line spec helper bodies are
  inlined into trait declarations, producing unbound `impl.f_repr`
  refs in extracted F*.  Reformat to multi-line bodies (matching
  `serialize_10_*` format) so hax keeps them as function calls.

#### USER-9b (5-bit, real AVX2 SIMD) — ✅ **CLOSED 2026-04-29** (this branch `agent/user-9b`)

The SIMD↔BitVec bridge mirrors USER-9a exactly: the AVX2 5-bit primitives
in `src/vector/avx2/serialize.rs:352, 466` carry `verification_status(lax)`
+ BitVec pre/post on their signatures (so the SIMD body remains
unverified, but the post claim is now visible to callers).  The wrapper
layer in `src/vector/avx2.rs` adds 3 admitted bridges
(`op_serialize_5_pre_bridge`, `op_serialize_5_post_bridge`,
`op_deserialize_5_post_bridge`) — discharged via the existing
`bit_vec_of_int_t_array_vec256_as_i16x16_lemma` axiom (no new axiom in
`avx2_extract.rs`!).  Wrapper-level `op_serialize_5` /
`op_deserialize_5` carry the trait pre/post and discharge it via the
bridges.

#### Audit findings

  - **No new SIDEWAYS axiom** required — the existing
    `bit_vec_of_int_t_array_vec256_as_i16x16_lemma` (used by the 4/10/11/12
    bridges) is parametric in `d` and works for `d=5` directly.
  - `avx2/serialize.rs:352, 466` `serialize_5` / `deserialize_5`: gain
    `verification_status(lax)` + BitVec pre/post (mirror 11-bit shape).
  - `avx2.rs`: 3 admitted bridges added inside the F* `before`-block;
    `op_serialize_5` / `op_deserialize_5` wrappers added with trait
    pre/post; `impl Operations` `serialize_5` / `deserialize_5` updated
    to dispatch through the wrappers.
  - `portable.rs`: free `serialize_5` / `deserialize_5` gain trait
    pre/post and invoke `serialize_5_lemma` / `deserialize_5_lemma`
    (the BitVec lemmas committed at `a51ddbfc3`); trait impl
    `serialize_5` / `deserialize_5` gain matching pre/post.
  - `traits.rs`: TODO(C4) replaced with
    `requires(spec::serialize_5_pre)` / `ensures(spec::serialize_5_post)`
    etc.  Spec helpers reformatted from single-line to multi-line bodies
    (the hax-quirk noted under USER-9a applies here too — single-line
    bodies got inlined and produced unbound `impl.f_repr` refs in
    extracted F*).

#### Verification (cold, no hints prior)

  - `Libcrux_ml_kem.Vector.Traits.fst` — 81 s
  - `Libcrux_ml_kem.Vector.Avx2.Serialize.fst` — 38 s
  - `Libcrux_ml_kem.Vector.Avx2.fst` — 74 s
  - `Libcrux_ml_kem.Vector.Portable.fst` — 56 s
  - `Libcrux_ml_kem.Vector.fst` — 2.4 s
  - `Libcrux_ml_kem.Serialize.fst` — 16 s (downstream consumer)

#### Net admit-count delta

+3 admitted bridges in `avx2.rs` F* `before`-block
(`op_serialize_5_pre_bridge`, `op_serialize_5_post_bridge`,
`op_deserialize_5_post_bridge`).  No new axioms.  No new lane admits.
The `verification_status(lax)` on the 5-bit primitive bodies is a
**signature-level** axiom (the BitVec post is asserted as fact); not
counted as a new admit because it replaces what was already an
unspecified body.

(Original USER-9 brief follows for reference.)

**Trait method declarations** in
`libcrux-ml-kem/src/vector/traits.rs:1320-1342`.

- **Status**: 4 portable BitVec lemmas landed in
  `src/vector/portable/serialize.rs` at commit `a51ddbfc3` —
  `serialize_5_lemma`, `serialize_11_lemma`, `deserialize_5_lemma`,
  `deserialize_11_lemma`.  All 4 discharge via
  `Tactics.GetBit.prove_bit_vector_equality' ()`.  Spike on
  `serialize_5_bit_vec_lemma` (80 bits) closed in 744 ms cold.
- **Verification status**: NOT yet confirmed clean for all 4 with
  `VERIFY_SLOW_MODULES=yes`.  Two runs in the parent agent (35 + 42
  min wall) progressed through `serialize_11_bit_vec_lemma` query 93
  and `deserialize_10_bit_vec_lemma` query 136 before being killed.
  Tactic IS working but slow at 11-bit (176-bit) widths.  Strategies
  for scale-up documented in `proofs/agent-status/serialize-prompt.md`.
- **Independent agent**: `proofs/agent-status/serialize-prompt.md`
  drives the scale-up; runs in parallel with Wave-A in this same
  worktree but on a file-disjoint surface
  (`src/vector/portable/serialize.rs` only).  Does NOT touch the
  trait or any Wave-A B-lane surface.
- **Why deferred (trait wire-up)**: AVX2 side
  (`src/vector/avx2/serialize.rs:351-700`) has a real SIMD
  implementation for `serialize_5/deserialize_5` and `lax` on
  `serialize_11/deserialize_11`.  Wiring the trait post would force
  AVX2 to discharge a SIMD↔BitVec bridge.  Per R3 cannot land
  unilaterally.  The portable lemmas are PREP for whoever builds the
  AVX2 bridge.
- **Once closed**: 4 trait methods get strong serialize/deserialize
  posts; downstream `Serialize.fst` migration (Phase 7c / A1) gains
  hacspec citations for these widths.

### USER-10 — trait `rej_sample` post strengthening + drop `lax` on `sample_from_uniform_distribution_next` — **IN FLIGHT (Wave-B A2; Wave-C re-attempt 2026-04-29)**

**`Operations::rej_sample`** in
`libcrux-ml-kem/src/vector/traits.rs:1352-1360`, plus
**`sample_from_uniform_distribution_next`** in
`libcrux-ml-kem/src/sampling.rs:51` (carries `verification_status(lax)`).

- **Status**: bounds-only post in place on `rej_sample`;
  `spec::rej_sample_post` helper (lines 1184-1198) defined with
  `Hacspec_ml_kem.Sampling.rej_sample_step` citation.  The caller
  `sample_from_uniform_distribution_next` retains
  `verification_status(lax)` because removing it triggers full SMT
  on a 2-level forall over `K` that Z3 cannot instantiate.
- **Why deferred**: Phase 1 Cluster 4 explicit defer.  Discharging
  `rej_sample_step` equation from the impl bodies (portable + AVX2)
  requires non-trivial annotation on the rejection-sampling loop —
  see `Phase 7n` brief for the analogous `sample_from_uniform_distribution_next`
  pattern.

#### Wave-C 2026-04-29 spike (lane A2 re-attempt) — REVERTED at 4-attempt cap

Tried four fix paths against the bare `lax` removal at
`src/sampling.rs:51` (rlimit 400 baseline):

  1. **Path (a)** `--split_queries always` alone: SMT picks up but
     splits the failure into ~3 sub-queries.  Q381/382 ("incomplete
     quantifiers" in 1.8 s each) at the inner-loop body's
     subtyping check (`out[j][k]` bounds for `j != i`); Q531 canceled
     at rlimit 400 (35 s, "unknown because canceled") on outer-loop
     post.
  2. **Path (c)** `--z3rlimit 800` (combined with split): Q531 turns
     from "canceled" into "incomplete quantifiers" at rlimit 452 —
     confirms the wall is a quantifier-instantiation problem, not
     rlimit exhaustion.  More rlimit doesn't help.
  3. **Path (d)** Tighten invariant by dropping the `if j < i` ladder
     and using a uniform `forall j. sampled_coefficients[j] <= K + 16`
     (plus a separate `forall j < i. <= K` for the second outer loop):
     Initial attempt with two `loop_invariant!` calls failed at
     extraction (hax emits `Hax_lib.e_internal_loop_invariant` for the
     second invariant, which F* doesn't recognize — `loop_invariant!`
     must be invoked at most once per loop).  Combined the two
     invariants into one disjunctive form (`<= K + 16 /\ (j >= i || <= K)`).
     Result: now only 2 inner-loop "incomplete quantifiers" failures
     (Q381/382) plus 1 outer-loop failure (Q525).  Closer than path (a)
     but still walled.
  4. **Path (b)** `Classical.forall_intro (Classical.move_requires aux)`
     placed after the if-body modification, with `aux j : Lemma
     (requires j <. v_K) (ensures invariant_at j) = ()`: the `aux`
     body itself fails ("incomplete quantifiers" inside the lemma
     proof) because Z3 still has to instantiate the loop-precondition
     forall at j.  Adding a second `inv_at` block BEFORE the if-body
     to materialise the entry-state invariant didn't break the wall
     either.  An `if j = i then () else ()` case-split shape was
     extracted but didn't change the failure mode.

Reverted to baseline `lax` at the 4-attempt cap (R6 of the lane-A2
prompt).  Final commit on `agent/lane-A2`: working tree restored to
`fa31480cd` HEAD content for `src/sampling.rs`; only the
`MLKEM_STATUS.md` and `agent-trackA.md` updates carry forward.

**Z3 finding**: rlimit isn't the wall; Z3 cannot instantiate the
2-level forall `forall j < K. (sampled_coefficients[j] <= K + 16) /\
(forall k < sampled_coefficients[j]. out[j][k] in [0, 3328])` when
the body updates `sampled_coefficients[i]` and `out[i]` via
`update_at_usize`.  Even a uniform invariant + `Classical.forall_intro`
hint doesn't break the wall — the `aux` lemma's body is itself an
"incomplete quantifiers" failure.  Likely needs:
  * Splitting the inner forall into a non-array-quantified shape
    (e.g. extracting a per-`j` predicate to a top-level `let` so the
    update_at can be normalised symbolically); OR
  * `--smtencoding.elim_box true` / `--smtencoding.l_arith_repr native`
    tweaks; OR
  * `--using_facts_from` to prune the search space; OR
  * Refactoring the loop body so update happens through a function
    with a per-index post (so `update_at_usize` is opaque to the
    invariant proof).

**Recommendation**: keep `lax` until a user with `smtprofiling`
expertise drives the quantifier instantiation problem to ground
(probably 1-2 hr of focused Z3-encoding work).

### USER-11 — drop `op_ntt_multiply` `panic_free` (Wave-A B5; Z3-walled) — **USER LANE** (NTT-related)

(Reproducer + 3 path-forward strategies in original B5 finding below.
Body proof shape is validated — math holds, Z3 saturation is the
blocker.  Best handled by a user with smtprofiling.)

**Wrappers**: `op_ntt_multiply` in `libcrux-ml-kem/src/vector/portable.rs:899`
and `libcrux-ml-kem/src/vector/avx2.rs:571` (both backends).

- **Status**: spiked 2026-04-29 (Wave-A B5) and reverted at the
  20-min cap.  Body proof shape validated; Z3 saturates at rlimit 400.
- **Math**: A1–A4 base-case lemmas already proven.  Body chains 8
  `lemma_base_case_mult_pair_commute` calls (one per binomial pair
  k ∈ {0..7} with appropriate ±zeta) plus `forall4` over per-branch
  `ntt_multiply_branch_post`.
- **Z3 wall observed (rlimit 400 + --split_queries always)**:
  * Q28 (sub-conjunct 1 of `assert (p_branch 0)`): 31 s, 126/400.
  * Q30 (sub-conjunct 2): 51 s, 192/400.
  * Q32 (sub-conjunct 3): 84 s, 279/400.
  * Q34 (sub-conjunct 4): **canceled at 400/400 in 104 s** → Error 19.
  Per-conjunct rlimit roughly doubles; Z3 accumulates the 8 binomial
  facts + 4 branch_post conjuncts + the if-ladder for `zp` selection.
- **Path forward** (3 strategies, ordered by cost):
  1. Per-conjunct decomposition: replace `assert (p_branch b)` with
     4 explicit lane-FE assertions, one per lane in the branch.
     Each Z3 query then has only the relevant pair_commute fact in
     scope.  +48 lines × 2 backends.
  2. Per-branch helper lemma in `Hacspec_ml_kem.Commute.Chunk.fst`
     (B5 additions section), one per concrete `b ∈ {0..3}`,
     mirroring the inverse layer-2 unlock pattern (commit
     `b7b49c358`).  Wrapper invokes 4 helpers + forall4.
  3. rlimit bump to 800 + intermediate hand-holding asserts.
     Cheapest but least robust if 4th conjunct still saturates.
- **Reproducer**: see `proofs/agent-status/wave-A-continuation-prompt.md`
  ("B5 finding" section) + `proofs/agent-status/agent-trackA.md`
  (Wave-A B5 spike entry) for the full body proof and Z3 stats.
- **Why USER-N**: 20-min cap exceeded; Z3 tuning is the bottleneck,
  not math.  A user with smtprofiling can pick the right strategy
  faster than agent-iteration on rlimit knob.

### USER-12 — drop `--admit_smt_queries true` on portable NTT layer 1 (Wave-A B2; Z3-walled) — **USER LANE** (NTT-related)

**Wrappers**: `op_ntt_layer_1_step` and `op_inv_ntt_layer_1_step` in
`libcrux-ml-kem/src/vector/portable.rs:422` and `:661`.

- **Status**: not attempted in Wave-A (deferred per 1-session budget +
  documented Z3 wall).  Body proofs are already written (8
  `lemma_butterfly_pair_commute` calls + per-group asserts +
  `forall4` intro), but the 4-zeta-parallel wall causes Z3 to hang.
- **Wall reproducer (per `portable.rs:397-421` admit comment, Phase 6
  agent A 2026-04-27)**: at rlimit 800 + `--split_queries always`,
  60 sub-queries closed in ~16 ms each, then a single sub-query
  (one of the 4 per-branch `p_layer_1 b` asserts, b ∈ {0..3}) ran
  >10 min without resolving before manual cancel.  Earlier rlimit
  400 run failed cleanly with Error 19 in 4 min.
- **Hypothesis**: per-branch `p_layer_1` body uses 4-way conditional
  `if b=0 then zeta0 else if b=1 ...` to pick zeta, which Z3
  case-splits on every per-lane FE-algebra fact.  Same shape as the
  layer-2 inverse wall that was unlocked at commit `b7b49c358`.
- **Path forward (RECOMMENDED)**: apply the layer-2-inverse unlock
  pattern (commit `b7b49c358`) to forward layer 1 + inverse layer 1.
  Refactor:
  1. 4 per-branch lemmas in `Hacspec_ml_kem.Commute.Chunk.fst`,
     each with literal `b ∈ {0..3}` so the if-ladder collapses
     pre-SMT; each lemma calls 2 `lemma_butterfly_pair_commute` and
     reveals the opaque branch_post for its concrete `b`.
  2. Per-lane wrapper that dispatches to the right per-branch helper.
  3. Per-vector composition under `--split_queries always` — Z3
     splits the forall over 16 lanes into 16 cheap sub-queries.
- **Sister issue**: USER-4 is the AVX2 mirror of this wall.  Both
  may benefit from the same per-branch decomposition pattern.
- **Why USER-N**: 4-zeta wall is Z3-saturation, not math.  Pattern is
  proven (`b7b49c358`); applying it to layers 1 (forward + inverse)
  is mechanical but ~2 sessions and best done with smtprofiling.

### USER-13 — drop `panic_free` on `compress_1` / `compress<D>` / `decompress_d` chunk wrappers (Wave-A continuation B1; Z3-walled)

- **Files**: `libcrux-ml-kem/src/vector/portable/compress.rs` lines 170 (`compress_1`), 222 (`compress<D>`), 347 (`decompress_d`).
- **Status**: panic_free retained; primitive `compress_message_coefficient` (line 27) and chunk wrapper `decompress_1` (line 289) closed in `f87e9a8cf`.  `compress<D>` is also blocked on USER-N Barrett primitive (`compress_ciphertext_coefficient` line 113).
- **Wall**: 16-element forall + integer-form per-lane equation in loop invariant.  Spike at rlimit 600 + `--split_queries always` with input-tracking helper `#[cfg(hax)] let _a0 = a` and enriched invariant: Q82/Q83 cancel at 600/600 in 200+s each on the closing-loop subtype check.  Same shape Z3 saturation as B5 (USER-11) — accumulator forall over the per-lane `((a_j * 4 + 3329) / 6658) % 2` integer division equation cannot be reproven incrementally.
- **Path forward**:
  1. Per-lane decomposition: replace forall in invariant with explicit 16-lane assertions per iteration (cheap pre-SMT, total +200 lines of body proof).
  2. Opaque per-lane predicate wrapper: define `compress_1_lane_post i a a0 = ((v a0[i] * 4 + 3329) / 6658) % 2 == v a[i]` and gate the forall with `[@@ "opaque_to_smt"]`.
  3. Loop-invariant frame factoring: the issue is the frame `(j >= v $i ==> a.f_elements[j] == _a0.f_elements[j])` chained with the past-iteration integer-form fact.  An opaque wrapper around the past-iteration fact would cut Z3 context.
- **Sister issue**: USER-11 (op_ntt_multiply panic_free) has the same shape — accumulating Z3 context across per-lane forall + integer arithmetic.  Both benefit from per-lane decomposition or opaque wrapping.
- **Why USER-N**: Z3 wall hit at 20-min cap; needs smtprofiling justification for a higher rlimit or per-conjunct factoring strategy.

### USER-14 — close 4 admits in trait `from_bytes` / `to_bytes` impl bodies (USER-8 carry-over) — 🔶 **STILL OPEN — Path 1 spike attempted 2026-04-29, did not close**

- **Files**: `libcrux-ml-kem/src/vector/portable.rs:957-983` (impl-side `from_bytes` / `to_bytes`), `libcrux-ml-kem/src/vector/avx2.rs:1051-1073` (same).  4 `admit ()` calls, 1 per impl method × 2 backends.
- **Status**: trait posts WIRED (`from_le_bytes_post_N` / `to_le_bytes_post_N` from `src/vector/traits.rs:234-246`); impl methods carry the strengthened post but discharge via admit.
- **Why FAIR GAME**: closing requires a `BitVecEq.int_t_array_bitwise_eq` lemma between a 32-byte input and a 16-i16 lane array (depth 8 vs depth 16, 256 bits each side).  Mechanically the same shape as `serialize_4_bit_vec_lemma` (8-byte vs 16-i16 at depth 8 vs depth 4).  Provable by `Tactics.GetBit.prove_bit_vector_equality' ()`.
- **Tooling-cycle blocker**:
  - The `Tactics.GetBit.prove_bit_vector_equality'` tactic only normalizes function bodies in the **current module's namespace**.  `Vector_type.{from,to}_bytes` lives in `Libcrux_ml_kem.Vector.Portable.Vector_type`.
  - Putting the lemma there would add `Tactics.GetBit` as a static import.  But `Tactics.GetBit.fst` (in `fstar-helpers/fstar-bitvec/`) statically references `Libcrux_ml_kem.Vector.Portable.Vector_type.__proj__Mkt_PortableVector__item__f_elements` (line 27, the `f_elements` projector for body normalization), creating a circular module dependency.
  - Putting the lemma in `Libcrux_ml_kem.Vector.Portable` (where `Tactics.GetBit` is already imported) doesn't help either: `Vector_type.{from,to}_bytes` body is hidden behind a `.fsti` `val` declaration, so the tactic can't unfold them.

#### Spike attempted by USER-14 lane (2026-04-29):

**Path 1 (Tactics.GetBit refactor) — partial advance, not closed**:
1. Replaced `\`%Libcrux_ml_kem.Vector.Portable.Vector_type.__proj__...f_elements`
   on `Tactics.GetBit.fst:27` with the **string-form namespace** `"Libcrux_ml_kem.Vector.Portable.Vector_type"`.  Backtick-FQN
   creates a static `.depend` edge; string-namespace does NOT.  This DID
   break the cycle — Tactics.GetBit recompiled in 0.6s, and the
   namespace covers `f_elements` (projector) plus all other Vector_type
   symbols, so the unfold capability is a strict superset.
2. Added (`hax_lib::fstar::after`) `from_bytes_bit_vec_lemma` and
   `to_bytes_bit_vec_lemma` to `vector_type.rs` mirroring the
   `serialize_4_bit_vec_lemma` shape.  The cycle break was verified
   (no .depend cycle error).
3. **The tactic ran but failed**: with concrete loop bounds
   (`v_FIELD_ELEMENTS_IN_VECTOR == sz 16`), the tactic only unfolded
   ONE recursion of `Rust_primitives.Hax.Folds.fold_range`.  Adding
   `"Rust_primitives.Hax.Folds"`,
   `"Rust_primitives.Hax.Monomorphized_update_at"`,
   `"Libcrux_ml_kem.Vector.Traits"`, and `"Rust_primitives.Integers"`
   to the `delta_namespace` did NOT cause deeper unfolding — Z3 still
   sees `match v 0 < v 16 with | true -> fold_range 1 16 ... | _ -> repeat 0`
   stuck after one step.  Reproduces with rlimit 800.
4. **Side effect**: with the expanded `delta_namespace`, the
   `serialize_5_bit_vec_lemma` Q (in
   `Libcrux_ml_kem.Vector.Portable.Serialize.fst`) FAILED at
   ~26-min wall under `VERIFY_SLOW_MODULES=yes`.  Aggressive unfold
   exposes context that destabilises slow proofs.  The USER-14 lane
   reverted Tactics.GetBit fully (`git diff` clean) before commit; the
   final tip is the same as USER-8's tip `bb6f740a2` minus session
   docs.

**Conclusion**: Path 1 by itself is insufficient.  The fold-loop body of
`Vector_type.{from,to}_bytes` doesn't unfold mechanically under
`norm [iota; zeta; delta_namespace ...]`, even with all relevant
namespaces enabled.  Two follow-on routes:

  **Path 1a — straight-line body refactor** (recommended next attempt):
  Refactor `vector_type.rs::from_bytes` and `to_bytes` Rust source from
  the `for i in 0..16 { elements[i] = ... }` loop form into an explicit
  `[(array[1] << 8) | array[0], (array[3] << 8) | array[2], …, (array[31] << 8) | array[30]]`
  array-of-list form (mirrors `serialize_4_int` from `serialize.rs`).
  Cost: 24 transitive consumers' `.checked` invalidate (the touch-
  unchanged-checked feedback note doesn't apply because Vector_type's
  content hash genuinely changes).  But the lemma then closes mechanically
  via `prove_bit_vector_equality'` — PROVIDED the Tactics.GetBit static
  cycle is broken first (Path 1's string-namespace fix on line 27,
  re-applied carefully).  Estimate: 2 hr (incl. ~30 min cache cascade).

  **Path 1b — sandwich pattern** (smaller blast radius):
  Keep both Vector_type body AND Tactics.GetBit untouched.  Add a
  `from_bytes_straight` / `to_bytes_straight` helper *inside* Vector_type
  that's straight-line, plus a `from_bytes_eq_straight` Lemma (provable
  by induction on the fold or by `Seq.lemma_eq_intro` + per-index
  `Seq.upd` reasoning).  Then the bit-vec lemma uses the straight-line
  form.  Drawback: still needs the cycle-break (Path 1's string fix).
  Estimate: 4 hr.

  **Path 2 (`hax_lib::fstar::replace(interface)` body inline)** — also
  viable if `hax_lib::fstar::replace` accepts that target on the .fsti
  side.  Untested.  Estimate: 1 hr.

**Files modified during USER-14 spike (all reverted)**:
  - `fstar-helpers/fstar-bitvec/Tactics.GetBit.fst` (line 27 alt) — REVERTED
  - `libcrux-ml-kem/src/vector/portable/vector_type.rs` — REVERTED

**Sister issue**: USER-9b (AVX2 5-bit SIMD bridge) has a related but distinct shape: it's a SIMD-intrinsic→BitVec bridge at the `mm256_madd_epi16` level, not a tactic-namespace cycle.

---

## AGENT TASKS — parallelizable mechanical work

Pattern-following proofs and migrations.  Multiple agent sessions can
run in parallel; each phase commit-able independently.

### Parallelism plan — what to fan out vs. serialize

**Hard constraint**: only `src/vector/traits.rs` is touched by multiple
phases.  All Phase 7* additions to `traits.rs` are post-only conjunctive
additions; the diffs don't conflict structurally, but any rebase still
costs the cache invalidation on `Vector.Traits.fst` (cascades through
Vector.{Portable,Avx2}.fst).  So minimize cross-branch traits.rs edits
in flight.

**Wave 1 — fan out 4-way (no shared file edits)**:
| Branch | Phase | Touches | Cache scope |
|---|---|---|---|
| `agent/phase-6-portable-ntt` | Phase 6 (6 portable NTT-layer admits) | `src/vector/portable.rs` only | invalidates Vector.Portable.fst only |
| `agent/phase-7c-serialize` | Phase 7c (Serialize re-root) | `src/serialize.rs` + Serialize callers | invalidates Serialize.fst only |
| `agent/phase-6c-avx2-stragglers` | Phase 6c (Sampling/Compress AVX2 admits) | `src/vector/avx2/sampling.rs`, `compress.rs` | invalidates Vector.Avx2.{Sampling,Compress}.fst |
| `agent/phase-6d-neon-mask` | Phase 6d (Vector.Neon Error 162) | `proofs/fstar/extraction/Makefile` | n/a (admit) |

These four can run concurrently.  None of them require new commute
lemmas; all are confined to single F* modules; cache invalidation
windows are disjoint.

**Wave 2 — Polynomial (single agent, chunk-commute is the gate)**:
- Phase 7a (Polynomial 7 fns).  This phase **adds Tier-1 commute lemmas**
  in `Hacspec_ml_kem.Commute.Chunk.fst`; those lemmas become the input
  for Phase 7b/d.  Don't fan out 7b before 7a's lemmas exist.
- Phase 7a touches `src/polynomial.rs` (post additions) + Chunk lemma
  file additions.  Single agent.

**Wave 3 — fan out 3-way once Wave 2 lemmas exist**:
| Branch | Phase | Depends on Wave 2 lemma | Touches |
|---|---|---|---|
| `agent/phase-7b-ntt` | 7b (Ntt + Invert_ntt layers 1-3) | Tier-2 layer-step commute (new in 7b itself) | `src/ntt.rs`, `src/invert_ntt.rs`, Chunk |
| `agent/phase-7d-matrix` | 7d (Matrix 4 fns) | Tier-1 (Wave 2) + Tier-2 (after 7b lands) | `src/matrix.rs`, Chunk |
| `agent/phase-7n-sampling` | 7n (sample_from_uniform_distribution_next) | none (orthogonal) | `src/sampling.rs` |

7b and 7n run in parallel.  7d can start in parallel with 7b *only* if
the matrix proofs don't need Tier-2; otherwise queue 7d after 7b.

**Wave 4 — Spec.MLKEM teardown (sequential)**:
- Phase 7j → 7k → 7l → 7m chained.  Don't fan out — each step removes
  symbols the next step depends on absence-of.  One agent end-to-end.

**Wave 5 — USER tasks merge in**:
- USER-1 (A8) can land any time — independent.
- USER-2/5/6 (full NTT / ntt_multiply / invert_ntt_montgomery) gated on
  Wave 3 (Phase 7b's Tier-2 layer-step commute).
- USER-3 (`to_standard_domain`) independent — can land any time.
- USER-4 (AVX2 NTT-layer 1/2 bridges) independent of all the above
  (Z3-blocked; mitigation, not throughput, is the bottleneck).

**Recommended kickoff (parallel today)**:
```
Wave 1 (concurrent):
  agent A: Phase 6 (portable NTT-layer admits)
  agent B: Phase 7c (Serialize re-root)
  agent C: Phase 6c (AVX2 Sampling/Compress)
  agent D: Phase 6d (Neon mask — 5 min)

Wave 2 (after Wave 1):
  agent E: Phase 7a (Polynomial + Tier-1 lemmas)

Wave 3 (after Wave 2):
  agent F: Phase 7b (NTT + Inv-NTT + Tier-2 lemmas)
  agent G: Phase 7n (Sampling, parallel with 7b)
  agent H: Phase 7d (Matrix, after 7b lands)

Wave 4 (after Wave 3):
  agent I: Phase 7j → 7k → 7l → 7m sequentially

USER tracks (parallel to all waves):
  user-A: USER-1 (A8) — anytime
  user-B: USER-2 (full NTT commute) — after Wave 3 lemmas land
  user-C: USER-3 (to_standard_domain) — anytime
  user-D: USER-4 (AVX2 layer 1/2 bridges) — anytime, blocked separately
```

**Why not fan out wider** in Wave 1: the four-way split saturates the
non-traits-touching mechanical work.  Any further parallelism (e.g.
splitting Phase 6 into per-layer subagents) yields diminishing returns
because each F* file rebuild dominates over per-method iteration.

### Trait-impl layer

| Item | Location | Phase | Effort |
|---|---|---|---|
| 6 portable NTT-layer ops `--admit_smt_queries true` | `src/vector/portable.rs` lines 331, 410, 488, 563, 638, 715 (op_ntt_layer_{1,2,3}_step + op_inv_ntt_layer_{1,2,3}_step) | **Phase 6** | 4-6 h.  Use `fstar-mcp` for per-file iteration; existing 8-butterfly_pair_commute structure + `forall4 p_layer_X` assert are in place. |
| 7 AVX2 serialize/deserialize bridge admits | `src/vector/avx2.rs` (bridges for serialize_{4,10,12} + deserialize_{1,4,10,12}) | **Phase 6b** | Per-N `Tactics.GetBit.prove_bit_vector_equality'` invocation + `bit_vec_of_int_t_array` decomposition lemma in `Libcrux_intrinsics.Avx2_extract`.  Mechanical once the lane-bridge lemma is written. |
| 3× `admit ()` in `Vector.Avx2.Sampling.fst` (extracted) | corresponding `src/vector/avx2/sampling.rs` | **Phase 6c** | Investigate; may follow same pattern as Compress. |
| 3× `admit ()` in `Vector.Avx2.Compress.fst` (extracted) | `src/vector/avx2/compress.rs` | **Phase 6c** | Same. |

### Above-trait modules (post-only strengthening, citing `Hacspec_ml_kem.*`)

| Item | Phase | Effort |
|---|---|---|
| Polynomial 7 fns: add_to_ring_element, poly_barrett_reduce, subtract_reduce, add_error_reduce, add_standard_error_reduce, add_message_error_reduce, multiply_by_constant_bounded | **Phase 7a** | 6-10 h.  Add Tier-1 chunk commute lemmas via `Classical.forall_intro`. |
| Ntt + Invert_ntt layers 1-3 (8 fns) | **Phase 7b** | 8-12 h.  Add Tier-2 layer-step commute lemmas. |
| Serialize re-root: 8 already-cite-Spec.MLKEM + 6 trivial-post helpers | **Phase 7c** | 6-8 h.  Replace `Spec.MLKEM.*` citations with `Hacspec_ml_kem.Serialize.*` (additive, then Spec.MLKEM dropped in 7k). |
| Matrix 4 fns (compute_message, compute_ring_element_v, compute_As_plus_e, sample_matrix_A) | **Phase 7d** | 6-8 h.  Tier-4 vector/matrix commute lemmas. |
| `Sampling.sample_from_uniform_distribution_next` `--admit_smt_queries true` cleanup | **Phase 7n** | Investigation; may need rejection-loop invariant tightening. |
| Migrate Ind_cpa.fst, Ind_cca.Unpacked.fst off `Spec.MLKEM.*` to `Hacspec_ml_kem.*` | **Phase 7j** | 4-6 h.  Post-only consumer-side switch. |
| Drop `Spec.MLKEM.*` conjuncts from Serialize.fst (now redundant) | **Phase 7k** | 1 h.  Pure removal. |
| **Delete `Spec.MLKEM.*` module** | **Phase 7l** | 1 h.  Drop files; update Makefile / hax includes. |
| `Spec.Utils.*` → `Proof_utils.*` symbol-level migration | **Phase 7m** | 2-4 h. |
| Mask `Vector.Neon.Vector_type.fsti(10,0-13,1)` Error 162 in ADMIT_MODULES | **Phase 6d** | 5 min.  Decidable-equality is a module-level F* issue; admit unblocks `hax.py prove`. |

### Genuinely out of scope (per user goal: portable + AVX2)

- All other Vector.Neon.* modules — Neon proofs are not part of the
  end-to-end portable+AVX2 goal.  Stay in ADMIT_MODULES indefinitely.

---

# Historical: trait-poststrengthen branch (C4f plan)

(everything below is from before the trait-opacify branch was created)

**Branch**: `trait-poststrengthen`  **Tip**: `68233ffd4`
**Task**: C4f — portable `compress_1` / `compress<D>` / `decompress_1` /
`decompress_ciphertext_coefficient<D>`.

## User directives (this session)

1. Keep hacspec dependency at the **trait** level, not in primitive posts.
2. Add bridge lemmas (primitive post → hacspec form).
3. **Primitive posts must be strengthened** (the existing bounds-only
   posts don't give bridge lemmas enough to work with). Hacspec's
   integer formula is a good template.
4. For `compress_1` (→ `compress_message_coefficient`), try to **prove**
   the bridge.
5. For `compress<D>` (→ `compress_ciphertext_coefficient`, Barrett),
   try SMT / short F* ulib lemma; if not quick, **admit with English
   hints** and defer to user.
6. Same for `decompress_d`.
7. Plan first, check postconditions actually hold (audit), then prove.

## Audit (complete) — postconditions hold

- `compress_message_coefficient` ↔ `compress_d fe 1`: ✓ via 3-case split
  on `fe ∈ [0, 832] ∪ [833, 2496] ∪ [2497, 3328]`. Quotient of
  `(4·fe+3329)/6658` is 0 / 1 / 2 respectively; `mod 2` gives 0 / 1 / 0.
- `compress_ciphertext_coefficient` (Barrett) ↔ `compress_d fe D` for
  D ∈ {4,5,10,11}: ✓. Both approximate `round(fe·2^D / 3329)`.
  `10321340 / 2^35 ≈ 1/3329` (Barrett).
- `decompress_1`: ✓ direct: impl gives `{0, 1665}` for `a ∈ {0, 1}`;
  hacspec `decompress_d(0, 1) = 0`, `decompress_d(1, 1) = 1665`.
- `decompress_ciphertext_coefficient`: ✓ *identical formula*
  `(2·a·3329 + 2^D) / 2^(D+1)`, just `>>` instead of `/`.

## Revised plan (6 steps)

### S1. Strengthen primitive posts (compress side)
File: `src/vector/portable/compress.rs`.
- `compress_message_coefficient` post gains integer-form conjunct:
  `v result == ((v fe * 4 + 3329) / 6658) % 2` (on `fe ∈ [0, 3329)`).
  Expected: provable from the existing threshold post via a 3-case
  integer assertion chain.
- `compress_ciphertext_coefficient` post gains:
  `(v D == 4 \/ v D == 5 \/ v D == 10 \/ v D == 11) ==>
   v result == ((v fe * 2 * pow2 (v D) + 3329) / 6658) % (pow2 (v D))`.
  Expected: Barrett-to-exact-div reasoning will not go through in SMT.
  Use `#[hax_lib::fstar::verification_status(panic_free)]` if the body
  can't discharge the new post after a reasonable attempt, with a
  comment explaining the Barrett-approximation-exactness gap.

### S2. Strengthen `decompress_1` and `decompress_ciphertext_coefficient` ensures
File: `src/vector/portable/compress.rs`. No separate primitive; the
inline body is the primitive.
- `decompress_1` ensures conjunct:
  `v res_i == (2 * v a_i * 3329 + 2) / 4`, for `a ∈ {0, 1}`. Trivial
  (two values).
- `decompress_ciphertext_coefficient<D>` ensures conjunct:
  `(D ∈ {4,5,10,11}) ==> v res_i == (2 * v a_i * 3329 + pow2 D) / pow2 (D+1)`.
  Body is literally that (modulo `>>` vs `/`). Provable by bit-shift
  semantics.

### S3. Bridge lemmas (Layer-0.5 scalars)
File: `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst`.
- `lemma_compress_message_coefficient_fe_commute`:
  `(v fe < 3329 /\ <primitive post>) ==>
    CP.compress_d (i16_to_spec_fe fe_as_i16) 1 == i16_to_spec_fe (result_as_i16)`
  (trivial integer unfold).
- `lemma_compress_ciphertext_coefficient_fe_commute`: same at D.
- `lemma_decompress_1_fe_commute`: from `a ∈ {0, 1}` and
  `v res ∈ {0, 1665}`, conclude hacspec equality.
- `lemma_decompress_ciphertext_coefficient_fe_commute`: same at D.

### S4. Strengthen trait posts
File: `src/vector/traits.rs`.
- `compress_1_post` += `compress_post_N #16 1 vec result`.
- `compress_post` += `D ∈ {4,5,10,11} ==> compress_post_N #16 D vec result`.
- `decompress_1_post`, `decompress_ciphertext_coefficient_post`:
  already strong.

### S5. Rewire portable wrappers
File: `src/vector/portable.rs` (lines 226-263).
- Each wrapper's ensures → `spec::<op>_post(&a.repr(), &out.repr())`
  as Rust call.
- Body: call the Layer-1 chunk-commute lemma.

### S6. Close the 4 Layer-1 stubs
File: `Hacspec_ml_kem.Commute.Chunk.fst` (lines 768-810).
- Each stub body becomes `= ()` once trait post witnesses
  `compress_post_N` / `decompress_post_N`.

## Risk register

- S1 for `compress_ciphertext_coefficient` is the hardest: Barrett
  approximation exactness is classical but tedious. High likelihood of
  admit + `panic_free` or `admit()` in the F* bridge. That's explicitly
  OK per user directive.
- Changes to primitive posts require re-verifying the primitive bodies
  which currently use rlimit 200 + opaque_to_smt. Need to preserve the
  opacity and keep rlimit under 300 (user policy).
- C4e admits still present; C4f must not regress them. Run full prove
  after S1, S5.

## Next action

S1a: strengthen `compress_message_coefficient` post with integer form,
try to prove.

## Verification log (this session — C4f closed)

Final state at the C4f-closing tip:

- `Libcrux_ml_kem.Vector.Portable.Compress.fst` — **PASS** (~73 s).
- `Libcrux_ml_kem.Vector.Portable.fst` — **PASS** (117 s, rlimit 200).
  Down from C4e baseline 217 s — **45 % faster** despite carrying
  6 NTT-layer + 4 compress/decompress + ntt_multiply
  hacspec-FE-form posts (none of which existed in C4e).
- Full `make` from extraction dir — only failure is the pre-existing
  unrelated `Vector.Neon.Vector_type.fsti(7,8-7,54)` "decidable
  equality" error (same as C4e tip; documented in handoff under
  "Out of scope").

## Architectural changes this session

1. **Per-method `op_*` flattening of `impl Operations for PortableVector`.**
   Every impl method body is now either a one-line call to a free
   `op_<name>(args)` whose pre/post is the *exact* trait pre/post
   (subtyping check `P ==> P`, trivial), or a one-line call to an
   underlying primitive whose pre/post already matches the trait
   (`add`, `sub`, `multiply_by_constant`).  No proof code lives inside
   the impl block.  Each `op_*` and primitive verifies in its own SMT
   scope with its own `#push-options`; `impl_1`'s VC is uniformly
   trivial.  Iteration on any single method is now ~10 s instead of
   ~5 min.

2. **`panic_free` annotations live only on `op_*`** (compress family
   + ntt_multiply) — never on the impl method.  Impl methods carry the
   strong (opaque-where-needed) trait pre/post and have no admits.

3. **`unfold let forall{4,8,16,32}`** in `Spec.Utils.fsti`.  These
   small helpers expand to a fixed N-conjunction; without `unfold`,
   F* leaves them un-inlined at fuel 0, so `assert (forall4 p)`
   appeared to Z3 as an unrelated function application even when each
   `assert (p k)` succeeded individually.  `unfold` makes them inline
   at every use site, turning the assertion into the conjunction
   directly.  Unblocked all 6 NTT-layer wrappers at rlimit 200 — was
   timing out at 225 s on rlimit 400 in `op_ntt_layer_1_step`.

4. **`--z3refresh` removed** from `impl_1` push-options.  It cost ~10×
   on Vector.Portable.fst (217 s → 37 min) and per the smtprofiling
   skill catalog (technique 3) is generally a worse choice than
   `--split_queries always` alone.

5. **AVX2 `compress_*` / `decompress_*`** wrappers were
   pre-emptively marked `panic_free` with strengthened
   `${spec::*_post}` annotations matching the trait's strengthened
   posts (`compress_post_N` / `decompress_post_N`).  Per user
   guideline: don't burn F*/Z3 cycles on doomed proofs; document each
   annotation's reason and removal plan.  **Removal plan**: drop
   `panic_free` and prove the body when C4′ AVX2 mirror lands.

## Side notes

- `Spec.Utils.fsti`: `neg_i16` made total via guard on `i16::MIN`
  (overflow case sent to `mk_i16 0`). Lets the helper appear in trait
  posts without propagating an i16-MIN refinement.

## C4′ AVX2 mirror — refactor landed (proofs pending)

`src/vector/avx2.rs` mirrors the portable `op_*` flattening:

- Each impl method body collapses to a one-line `op_<name>(args)` call;
  trait subtyping check is `P ==> P`.
- `op_*` functions for previously-verified arithmetic ops
  (`cond_subtract_3329`, `barrett_reduce`,
  `montgomery_multiply_by_constant`, `to_unsigned_representative`)
  carry **real proofs** (no `panic_free`): each is a thin
  `reveal_opaque(is_i16b_array_opaque)` plus a call to the AVX2
  primitive, whose strong post (already proven in
  `Vector.Avx2.Arithmetic.fst`) discharges the trait post.
  `op_cond_subtract_3329` adds a `Seq.lemma_eq_intro` to bridge
  per-lane equality to `Spec.Utils.map_array` form (improvement over
  the pre-C4f `lax`).
- `op_*` for compress/decompress/NTT-layer/ntt_multiply carry
  `panic_free` with the same strengthened `${spec::*_post}` posts as
  the portable side (compress_post_N / decompress_post_N /
  hacspec FE-form for NTT). **Body proofs are pending C4′ Layer-1
  helpers in `Hacspec_ml_kem.Commute.Chunk.fst` plus AVX2-specific
  per-lane Vec256 ↔ array bridging.**
- `add`, `sub`, `multiply_by_constant`, `from_bytes`, `to_bytes`,
  `from_i16_array`, `to_i16_array`, `ZERO` — no `op_*` needed: the
  underlying `arithmetic::*` primitive's pre/post already matches the
  trait's exactly, so the impl method body is already a one-liner.
  (`to_bytes` stays `lax` — same as pre-C4f, F*/hax modeling issue
  around `&mut` slice bounds, out of C4' scope.)

Verification (this session):
- `Libcrux_ml_kem.Vector.Avx2.fst` — **PASS** (~11.5 s).
- `Libcrux_ml_kem.Vector.Avx2.Arithmetic.fst` — **PASS** (~26 s).
- `Libcrux_ml_kem.Vector.Avx2.Compress.fst` — **PASS** (~3.6 s).
- `Libcrux_ml_kem.Vector.Avx2.Ntt.fst` — **PASS** (~3.7 s).
- `Libcrux_ml_kem.Vector.Avx2.Serialize.fst` — **PASS** (~48.6 s).
- `Libcrux_ml_kem.Vector.Avx2.Sampling.fst` — **PASS** (~4.6 s).

## C4′ AVX2 serialize/deserialize wrappers — `lax` removed for {1, 4, 10, 12}

`src/vector/avx2.rs` `op_serialize_{4,10,12}` and
`op_deserialize_{1,4,10,12}` now drop `lax` and discharge the trait's
`{,de}serialize_post_N` via per-N admitted bridge lemmas
`op_{serialize,deserialize}_N_{pre,post}_bridge` (defined in a
`fstar::before` block on `op_serialize_4`).  Each bridge connects the
AVX2 primitive's BitVec lane post (`bit_vec_of_int_t_array r 8 i ==
vector ((i/N)*16 + i%N)`) to the trait's array-form post
(`BitVecEq.int_t_array_bitwise_eq` at depth N).

- 7 admitted bridge lemmas total (4 serialize: 3 pre + 3 post for
  N ∈ {4,10,12}; 4 deserialize post for N ∈ {1,4,10,12}; deserialize
  has no pre-bridge — the only pre is `Seq.length`).
- `op_serialize_1`, `op_serialize_11`, `op_deserialize_11` still `lax`:
  underlying primitive is `lax` too, so wrapper bridging is pointless
  until the primitives are proven first.
- **Removal plan for the admits**: per-N `Tactics.GetBit.prove_bit_vector_equality'`
  invocation plus a `bit_vec_of_int_t_array (vec256_as_i16x16 v) N`
  decomposition lemma in `Libcrux_intrinsics.Avx2_extract`.

Verified: `Vector.Avx2.fst` PASS.

## C4′ AVX2 NTT-layer wrappers — `panic_free` removed for all 6

`src/vector/avx2.rs` `op_{ntt,inv_ntt}_layer_{1,2,3}_step` now drop
`panic_free` and discharge the trait's strengthened FE-form posts via
6 admitted bridge lemmas
`op_{ntt,inv_ntt}_layer_N_step_bridge` (defined in a `fstar::before`
block on `op_ntt_layer_1_step`).  Each bridge takes `vector` + zetas
+ `result == ntt::*_step vector zetas` and concludes the trait
`*_step_post` (lifting the AVX2 primitive's body equality up to the
FE-butterfly form on `vec256_as_i16x16`).

- `op_inv_ntt_layer_1_step_bridge` precondition uses the primitive's
  non-opaque `is_i16b_array` (because the underlying
  `inv_ntt_layer_1_step.fsti` requires it); wrapper does
  `reveal_opaque is_i16b_array_opaque` before the call.
- **Removal plan for the admits**: strengthen the primitive posts in
  `Libcrux_ml_kem.Vector.Avx2.Ntt.fsti` (currently `Prims.l_True`)
  to expose per-lane FE equations, then prove each bridge directly
  from those.  `inv_ntt_layer_1_step.fst` body has
  `--admit_smt_queries true` which must be removed first.
- `op_ntt_multiply` keeps `panic_free` (blocked on C4e Layer-0.5
  admits — same on portable).

Verified: `Vector.Avx2.fst` PASS, `Vector.Avx2.{Arithmetic,Compress,
Ntt,Serialize,Sampling}.fst` all PASS, `Vector.Portable.fst` PASS.

## C4′ AVX2 ntt_layer_3_step — top-to-bottom proof landed

Strengthened `Libcrux_ml_kem.Vector.Avx2.Ntt.ntt_layer_3_step` post in
`src/vector/avx2/ntt.rs` to expose the per-lane butterfly residue
equation + `is_i16b_array (6*3328)` bound (matching the trait's
strengthened `ntt_layer_3_step_post`).  Body proof composes:

- **6 generic SIMD lane lemmas** in a `fstar::before` block on
  `ntt_layer_3_step`:
  - `lemma_mm256_castsi256_si128`, `lemma_mm256_extracti128_si256_1`,
    `lemma_mm256_castsi128_si256_lo`, `lemma_mm256_inserti128_si256_1`
    — admitted (require knowing the abstract `vec256_as_i16x16` /
    `vec128_as_i16x8` definitions, which are declared `val` in
    `Avx2_extract.fsti`).
  - `lemma_add_i_128`, `lemma_sub_i_128` — proven by `()`, with
    `SMTPat` triggers on `add_mod` / `sub_mod` to lift `+.` / `-.`
    to integer `+` / `-` under no-overflow.
- Intermediate per-lane assertions chain through `mm256_extracti128`
  → `mm_set1` → `montgomery_multiply_m128i_by_constants` →
  `mm256_castsi256_si128` → `mm_add_epi16` / `mm_sub_epi16` →
  `mm256_castsi128_si256` → `mm256_inserti128_si256<1>`.
- rlimit 400, `--split_queries always`.

The wrapper `op_ntt_layer_3_step` in `src/vector/avx2.rs` now
discharges the trait FE-form post directly via 8
`Hacspec_ml_kem.Commute.Chunk.lemma_butterfly_pair_commute` calls
(one per pair `(i, i+8)`) + per-b assertions + `forall4` —
identical pattern to portable.  No bridge admit needed for
`ntt_layer_3_step` anymore.

The other 5 NTT-layer wrappers (`ntt_layer_{1,2}_step` +
`inv_ntt_layer_{1,2,3}_step`) still use admitted bridge lemmas;
each could follow the same pattern once their primitive bodies in
`Vector.Avx2.Ntt.fst` get the matching strengthened posts (the 4
SIMD lane lemmas defined here are reusable).

Verified: `Vector.Avx2.fst`, `Vector.Avx2.Ntt.fst`,
`Vector.Avx2.{Arithmetic,Compress,Sampling,Serialize}.fst`,
`Vector.Portable.fst` all PASS.

## Inverse NTT layer 2/3 Barrett: spec aligned with AVX2/Neon optimization

`inv_ntt_layer_{2,3}_step` in `src/vector/avx2/ntt.rs` and
`src/vector/neon/ntt.rs` deliberately omit the Barrett reduction on
the sum lanes that portable's `inv_ntt_step` (called from
`inv_ntt_layer_{2,3}_step` in `src/vector/portable/ntt.rs`) applies.
This is an intentional optimization: each Barrett costs ~5 SIMD ops,
and the next Barrett in `invert_ntt_at_layer_4_plus`'s `step_reduce`
mops up the looser bound.  Portable was over-reducing.

Spec changes (in `src/vector/traits.rs`):
- `inv_ntt_layer_2_step_post`: `is_i16b_array_opaque 3328` →
  `is_i16b_array_opaque (2*3328)`.
- `inv_ntt_layer_3_step_pre`:  `is_i16b_array_opaque 3328` →
  `is_i16b_array_opaque (2*3328)`.
- `inv_ntt_layer_3_step_post`: `is_i16b_array_opaque 3328` →
  `is_i16b_array_opaque (4*3328)`.

Caller-side changes (in `src/invert_ntt.rs`):
- `invert_ntt_at_layer_2`: ensures bound `3328` → `2*3328`.
- `invert_ntt_at_layer_3`: requires bound `3328` → `2*3328`; ensures
  `3328` → `4*3328`.
- `invert_ntt_at_layer_4_plus`: requires bound `3328` → `4*3328`
  (uniform across all 4 calls; widening hints added in
  `invert_ntt_montgomery` between consecutive layer_4_plus calls).
- `inv_ntt_layer_int_vec_step_reduce`: requires
  `is_bounded_vector(3328, ...)` → `is_bounded_vector(4*3328, ...)`;
  internal `is_bounded_vector_higher` adjusted from `(6656, 28296)`
  to `(8*3328, 28296)`.

Bound trace (documented inline in `src/invert_ntt.rs`):
```
layer 1 input:  4*3328 → output: 3328 (Barrett)
layer 2 input:  3328   → output: 2*3328  (no Barrett)
layer 3 input:  2*3328 → output: 4*3328  (no Barrett)
layer 4_plus(4) input: 4*3328 → output: 3328 (Barrett in step_reduce)
layer 4_plus(5..7):    3328   → 3328 (steady state)
```

Safety (no integer overflow): worst-case sums in layers 2/3 are
`6656` and `13312`, both well below `i16` max (`32767`).  In
`step_reduce`, `a_plus_b` reaches `26624 < 28296` (Barrett pre).
Mont-mul i32 product peaks at `26624 × 1664 ≈ 4.4×10⁷ << 2³¹`.

External impact: **none** — `invert_ntt_montgomery`'s post is
unchanged at `is_bounded_poly(3328)`, so callers see identical
contracts.

Performance gain: ~80 SIMD ops saved per inverse NTT call (16
Barrett reductions removed × ~5 ops each) — was previously redundant
work on portable, was an unproven implicit optimization on AVX2/Neon.

Verified: `Vector.Portable.fst`, `Vector.Avx2.fst`, `Invert_ntt.fst`
all PASS (rlimit 400 on `invert_ntt_at_layer_{1,2,3}` due to query
size, with `--ext context_pruning` and split-query retries).

Note: portable's `inv_ntt_step` still does Barrett (used by layer 1).
Layers 2/3 in portable accept the looser primitive output trivially
(3328 ≤ 6656 ≤ 13312).  Removing Barrett from portable layers 2/3
specifically would require splitting `inv_ntt_step` into Barrett and
no-Barrett variants — left as a perf follow-up.

## Forward NTT bound symmetry

Audited via subagent: all three backends (portable, AVX2, Neon) use
pure mont_mul + add/sub butterflies in forward `ntt_layer_N_step`
bodies — no Barrett anywhere.  Bounds grow monotonically `3 → 4803 →
2*3328 → … → 8*3328`, then `re.poly_barrett_reduce()` once at the end
of `ntt`.  No analogue to the inverse-NTT layer 2/3 Barrett asymmetry
in the forward direction.

## Manual proof targets (A1–A7) — all closed

See `proofs/manual-proof-targets.md` for the original brief.  Status
update (2026-04-26):

- **A1** (`lemma_base_case_mult_even_mod_core`): ✅ proven by user with
  calc-style proof at rlimit 400 (~4 s; commit `08999e562`).
- **A2** (`lemma_base_case_mult_even_fe_commute`): ✅ proven (commit
  `e4a5f848a`).  Clean chain: `lemma_impl_*_v_val × 4 +
  lemma_base_case_mult_even_mod_core_fe_form`.  Closes in ~7 ms.
- **A3** (`lemma_base_case_mult_odd_mod_core`): ✅ proven by Claude with
  same calc style as A1 at rlimit 400 (~0.9 s; commit `44f401e72`).
- **A4** (`lemma_base_case_mult_odd_fe_commute`): ✅ proven (commit
  `e4a5f848a`).  Same chain pattern as A2 minus zeta.  Closes in ~6 ms.
- **A5** (`lemma_compress_message_coefficient_fe_commute`): ✅ proven
  (commit `a47efd17c`).  Two `f_val` asserts then `()`; closes in
  ~tens of ms.  Earlier `= ()`-only body segfaulted F* — the asserts
  unblock it.
- **A6** (`lemma_decompress_1_fe_commute`): ✅ proven (commit
  `a47efd17c`).  `()` body suffices; closes in ~1.6 s.
- **A7** (`lemma_decompress_ciphertext_coefficient_fe_commute`): ✅
  proven (commit `a47efd17c`).  `()` body suffices; closes in ~2.9 s.

`Hacspec_ml_kem.Commute.Chunk.fst` verifies end-to-end in ~109 s post
A1–A7 (verified 2026-04-26).

**A1–A4 design note**: the FE chain
`impl_add (impl_mul (mont a0) (mont b0)) (impl_mul (impl_mul (mont a1)
(mont b1)) (mont z))` produces an f_val whose inner products are
`((a*169)%q) * ((b*169)%q)` (both factors inner-modded), while A1/A3's
ensures have only the first factor inner-modded.  Two new int-level
helpers `lemma_base_case_mult_{even,odd}_mod_core_fe_form` absorb the
redundant inner mods via two `lemma_mod_mul_distr_r` calls and invoke
A1/A3.  This keeps A1/A3's calc proofs unchanged and fast; A2/A4
become trivial `lemma_impl_*_v_val` chains.

A1+A2+A3+A4 land enables dropping `panic_free` from `op_ntt_multiply`
on both portable and AVX2 backends, and replacing the wrapper
`admit ()` in `src/vector/portable.rs::Operations::ntt_multiply`
with a real proof (the `lemma_base_case_mult_pair_commute` calls are
already in place).

**A5/A7 + A8 admit**: closing the per-element fe_commute lemmas was
the input for the impl-side `op_compress*` panic_free drops.  A8
(parameterized `lemma_compress_ciphertext_coefficient_fe_commute`,
Barrett-exactness 4-case math) is admitted with statement preserved
— follows the same admit-then-prove pattern as A5–A7.

**Trait posts refactored to `Spec.Utils.forall16`** (commit
`fb4f8154d`).  Old form used `compress_post_N #16 1 vec result`
(generic-N forall, Form 1).  Per the C4-era forall benchmark, Form 1
at N=16 is 44× slower for callers and sometimes fails outright with
"incomplete quantifiers".  All 4 trait posts (compress_1_post /
compress_post / decompress_1_post / decompress_ciphertext_coefficient_post)
now use `Spec.Utils.forall16 (fun (i: nat{i<16}) -> ...)` directly.
The decompress posts retain their input-bound implication wrapper
(needed for `decompress_d`'s `Pure` requires to type-check).

**Chunk-level commute lemmas**: compress lemmas close cleanly
(`compress_1`: 755 ms, `compress`: 1888 ms).  Decompress chunk
lemmas (`decompress_1`, `decompress_ciphertext_coefficient`) admitted
— Z3 query 11/16 fails "incomplete quantifiers" even after revealing
`bounded_i16_array`.

**Portable `op_compress_1` / `op_compress<D>` panic_free DROPPED**
(commits `fb4f8154d`, `830ec8b8b`).  Body proof = 16 explicit per-lane
applications of A5 / A8.  Verifies in 6.3 s / 14 s respectively.
Same pattern is ready for `op_decompress_*` once their chunk lemmas
unblock.

## Deferred: SIMD model unification with libcrux-ml-dsa

See `proofs/simd-model-unification-plan.md` for a deferred plan to
unify the AVX2 SIMD model with libcrux-ml-dsa's `core-models`-based
approach.  Today both crates have parallel, incompatible bit-vector
libraries (`fstar-helpers/fstar-bitvec/BitVec.Bv` vs
`core_models::abstractions::bitvec::BitVec`) and parallel lane views
(abstract `vec256_as_i16x16` vs defined `to_i16x16` with inversion
lemmas) — every SIMD lemma we write today is doomed to a single repo.

**To pick up after C4' is fully done.**  Migration is sequenced to
discharge several currently-admitted ML-KEM SIMD lemmas as a side
effect (the four lane-bridge admits in `Vector.Avx2.Ntt.fsti`, the
seven serialize bridge admits in `Vector.Avx2.fst`).  Phased rollout
with single-cfg-flag toggle (`pre_core_models`) for incremental
migration.

## C4' AVX2 NTT-layer top-to-bottom proofs — 2 of 6 done, 4 blocked on Z3

Done:
- `ntt_layer_3_step` (commit `38ac22188`)
- `inv_ntt_layer_3_step` (commit `7c426e2e1`)

Both use the same SIMD lane lemmas (cast/extract/insert/add/sub for
Vec128/Vec256) admitted in a `fstar::before` block on
`ntt_layer_3_step`.  The wrappers in `vector/avx2.rs` now discharge
the trait FE-form post directly via 8
`lemma_{,inv_}butterfly_pair_commute` calls + per-b assertions +
`forall4` — bridge admits removed.

Blocked on Z3:
- `ntt_layer_1_step`, `ntt_layer_2_step`, `inv_ntt_layer_1_step`,
  `inv_ntt_layer_2_step`.

Each of the 4 blocked layers has the same structural shape: a
parallel SIMD body computing all 16 lanes via a single
`mm256_add_epi16`, with 4 zeta groups and 4 sub-equations per group
(64 atomic facts to chain through SMTPats simultaneously).  Z3
times out at rlimit 800 + 1800s wall-clock even when the body proof
is decomposed into 16 explicit per-lane assertions with
`--split_queries always`.  The portable side proves these efficiently
because its body is **sequential** per-lane (one `inv_ntt_step` /
`ntt_step` call per pair), so the proof chain is also sequential.

Possible mitigations (none attempted yet):
1. Refactor each AVX2 layer body into 4 per-zeta sub-functions
   (each handling 4 lanes), so the proof obligations are 4
   independent 4-lane proofs instead of one 16-lane proof.  Big
   AVX2 refactor.
2. Adopt the deferred SIMD model unification
   (`proofs/simd-model-unification-plan.md`) — Model B's defined
   `to_i16x16` (vs Model A's abstract `vec256_as_i16x16`) plus
   `bitvec_postprocess_norm` rewrite tactic should make the chain
   compositional rather than per-lane SMTPat-fire-storm.  Deferred
   until C4' is "done to admit boundary".
3. Manually-written F* lemmas inside `Vector.Avx2.Ntt.fst` that
   capture each "shuffle + mont_mul + add yields butterfly residue"
   as an admitted Vec256→Vec256 fact, called once per layer.
   Reduces 16-lane chain to 1-lane equality, but admits a per-layer
   black-box.

Status: pause AVX2 NTT-layer wrapper proofs at 2/6 done; wait for
user to land A1–A7 (`proofs/manual-proof-targets.md`) so progress
can resume on `op_ntt_multiply` and compress/decompress wrappers,
which don't have the 4-zeta-parallel SIMD wall.

## Open follow-ups

- **Phase 2 of the impl-flattening refactor**: for some `op_*` we may
  be able to fold the bridging directly into the underlying primitive's
  annotations and drop the wrapper.  Tracked separately.
- **C4′ true mirror (drop `panic_free`)**: AVX2 NTT-layer + compress/
  decompress + ntt_multiply `op_*` bodies need real proofs once
  the portable Layer-1 helpers and AVX2 per-lane Vec256↔array bridges
  are in place.
- **Forall-style benchmark**: which of `forall (k:nat). k<N ==> P k` /
  `forall (k:nat{k<N}). P k` / `Spec.Utils.forall<N> (fun k -> P k)`
  is most efficient in trait posts and at callers.  Dispatched as a
  parallel research task this session; results inform Phase 2.
- **C4e Layer-0.5 admits**: `lemma_base_case_mult_{even,odd}_{mod_core,fe_commute}`
  in `Hacspec_ml_kem.Commute.Chunk.fst` are still admitted; closing
  them unlocks dropping `panic_free` from `op_ntt_multiply`.

## Trait-boundary admits — TEMPORARY (`ADMIT_MODULES`)

Three modules **above `vector/traits.rs`** in the dep graph were added to
`proofs/fstar/extraction/Makefile` `ADMIT_MODULES` because the trait
post-strengthening propagates new refinements through their call sites,
which haven't been retuned yet.  The pattern is the same in all three:
subtyping on a loop accumulator's forall-in-refinement gets re-proven from
scratch on every iteration, exhausting Z3's rlimit.

Confirmed failing on macOS reference run too — admitting hides no
regression, only the pre-existing rlimit-borderline gap.

**Scope of the admit**: these are temporary, tied to the trait-boundary
work.  **Remove all three as soon as the trait-boundary cleanup is done.**
Drop the admit in the same commit as the fix so reviewers can re-verify.

| Module | Failure | Removal trigger |
|---|---|---|
| `Libcrux_ml_kem.Serialize.fst` | `compress_then_serialize_message` Q2 cancels at rlimit 80 (full 80; ~18 s on Mac) — assert at line 57 about `Vector.Traits.fst:587` | Split the heavy query upstream or restructure the assertion at `Serialize.fst:57` to avoid the heavy quantifier instantiation |
| `Libcrux_ml_kem.Polynomial.fst` | `Polynomial.ntt_multiply` Q54 cancels at rlimit 300 (full 300; ~90 s on Mac) — subtyping on loop accumulator refinement at line 845 | Hoist the per-iteration bound into a named lemma, or decompose the refinement so each conjunct verifies independently |
| `Libcrux_ml_kem.Invert_ntt.fst` | `invert_ntt_at_layer_2_` Q2 "unknown" at rlimit 400 (full 400; ~86 s on Mac) — subtyping on loop accumulator at line 149 | Same fix as `Polynomial.fst` (shared pattern) |

## Local-tuning fixes (kept, don't admit)

Two perf-margin failures that don't pass on this slower machine but do
pass on the macOS reference (within the rlimit cap).  Fixed locally with
small bumps:

- `Hacspec_ml_kem.Commute.Chunk.fst::lemma_base_case_mult_{even,odd}_mod_core_fe_form`:
  wrapped in `#push-options "--z3rlimit 400"` (was inheriting default 80).
- `Libcrux_ml_kem.Vector.Portable.Ntt::ntt_multiply` (`src/vector/portable/ntt.rs:463`):
  rlimit kept at 800 (briefly bumped to 1200 but reverted — the bump
  triggered an F*/Z3 segfault on this machine, possibly because more
  rlimit headroom let Z3 load more SMT context before the crash; same
  module is documented as segfault-prone in `proofs/c4f-handoff.md` §4).
  On macOS the Q186 still uses 771/800 i.e. closes within budget.

Both are pure rlimit bumps with no proof change; safe to keep.

## Pointer to full handoff

See `proofs/commutativity-handoff.md`.

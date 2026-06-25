# ML-DSA Verification — 48-Hour Sprint Plan

## Context

The libcrux ML-DSA (FIPS 204) implementation needs to be formally verified
against the hacspec at `specs/ml-dsa/`. The proof sister-effort for ML-KEM
on branch `trait-opacify` (tip `1c47e3632`) has matured a transferable
methodology: opaque per-lane / per-branch posts at the SIMD trait boundary,
post-only additive strengthening, `Classical.forall_intro` lifting, and
per-element commute lemmas bridging integer-arithmetic posts to hacspec
field-element form. ML-DSA must adopt the same architecture.

**First milestone (sprint goal)**: prove all 21 methods of the
`Operations` trait in `libcrux-ml-dsa/src/simd/traits.rs` for both
`portable` and `avx2` implementations, with **strong functional
postconditions** anchored to extracted `Hacspec_ml_dsa.*` modules — not
to the obsolete hand-written `Spec.MLDSA.Math.fst` / `Spec.MLDSA.Ntt.fst`.
ML-DSA has no Neon SIMD; only portable + AVX2 are in scope.

### State of the world (audited 2026-04-27)

| Area | State |
|---|---|
| Operations trait posts | **Bounds-only** for most methods; functional posts only on `add`/`subtract` (via `traits/specs.rs::add_post`) and `montgomery_multiply` (cites obsolete `Spec.MLDSA.Math.mod_q`). Most `requires(true)` and no `ensures`. |
| Hand-written F* spec | `proofs/fstar/spec/Spec.MLDSA.Math.fst`, `Spec.MLDSA.Ntt.fst`, `Spec.Intrinsics.fsti` — **obsolete, scheduled for deletion** per user direction. Cited by trait `montgomery_multiply` post and AVX2 NTT proofs. |
| Extracted F* (impl, `proofs/fstar/extraction/`) | **Done** — all 87 `Libcrux_ml_dsa.*.fst` files extracted. Files are gitignored (`.gitignore` = `*fst *fsti`). |
| Extracted F* (hacspec, `specs/ml-dsa/proofs/fstar/extraction/`) | **Done & typechecking** — `Hacspec_ml_dsa.{Arithmetic, Polynomial, Ntt, Sampling, Encoding, Matrix, Hash_functions, Parameters, Ml_dsa}.fst` all present and have `.checked` files in `.fstar-cache/checked/`. |
| Hacspec scalar helpers in `Hacspec_ml_dsa.Arithmetic` | `mod_q`, `mod_pm`, `decompose`, `high_bits`, `low_bits`, `make_hint` (returns bool), `uuse_hint`, `coeff_norm`. **Missing**: `power2round` (scalar), `shift_left_then_reduce`, `montgomery_reduce` — added in Phase 0D. |
| Hacspec polynomial helpers in `Hacspec_ml_dsa.Polynomial` | `poly_add`, `poly_sub`, `poly_pointwise_mul`, `poly_reduce`, `poly_mod_pm`, `poly_infinity_norm`, plus `vector_*` versions. NTT in `Hacspec_ml_dsa.Ntt.{ntt, intt, ntt_layer, intt_layer}`. |
| Hacspec sampling primitives | Polynomial-level `rej_ntt_poly`, `rej_bounded_poly`, plus per-byte step helpers `Hacspec_ml_dsa.Encoding.{coeff_from_three_bytes, coeff_from_half_byte}` — exactly what trait `rejection_sample_*` posts need. |
| Constants alignment | All match: q=8380417, D=13, γ₂∈{95232, 261888}, η∈{2,4}. |
| Shared scaffolding | `Spec.Utils` (`is_i32b_array_opaque`, `forall_N`, dual-trigger lemmas) already on the include path via `proofs/fstar/extraction/Makefile:36`. Has `forall4`, `forall16`; need `forall8`, `forall32`. |
| ML-KEM template docs | `libcrux-ml-kem/proofs/proof-style-guide.md`, `MLKEM_STATUS.md`, `outstanding-admits.md`, `manual-proof-targets.md` — directly transferable. |
| FIPS 204 audit | 33 findings: 0 Critical, 4 High, 12 Medium, 16 Low. Implementation conforms. Full report: `/Users/karthik/.claude/plans/curious-orbiting-swan-agent-aa190d3ffc7b63804.md`. |

## Strategy — copy ML-KEM's pattern

For each Operations method we want a structure of the form:

```fstar
(* trait::specs *)
[@@ "opaque_to_smt"] let X_lane_post (input result: i32) : prop =
   <equation citing Hacspec_ml_dsa.*>

let X_post (i o: t_Array i32 8) : prop =
   bound_predicate o /\ Spec.Utils.forall8 (fun i -> X_lane_post i.[i] o.[i])
```

The portable + avx2 impls call `reveal_opaque` on lane posts, then either
(a) call a per-lane commute lemma via `Classical.forall_intro` to lift to
the 8-element vector, or (b) for NTT/INVNTT, compose 4 butterfly commute
lemmas per branch behind a `forall4`-shaped post.

Three invariants we hold to (from ML-KEM proof-style-guide):
1. **Pre-conditions never touched** — only post conjunction added.
2. **Dual-trigger SMTPat** for consume direction; **named-intro lemma
   without SMTPat** for produce direction.
3. **Z3-divergence escape**: when a branch is intractable, admit the
   lemma, preserve the statement, document in `outstanding-admits.md`.

## 20-minute rule for proof attempts

**Hard wall-clock budget**: at most **20 minutes** per F* function/lemma
proof attempt. If the proof does not close within 20 minutes:

1. **Mark the function as admitted**, in one of two ways (preserving
   the statement):
   - Annotate the Rust function with `#[hax_lib::fstar::verification_status::panic_free]`
     (the proof obligation downgrades to "well-formed; no panic"), OR
   - Add `hax_lib::fstar!("admit()")` at the top of the function body
     (F* admits the obligation but keeps the type signature).
2. **Document** the failure in `libcrux-ml-dsa/proofs/outstanding-admits.md`:
   - File and line range of the function.
   - One-paragraph diagnosis (Z3 timeout? Quantifier instantiation
     blowup? Missing lemma? FE-algebra divergence?).
   - Suggested user-lane mitigation (e.g., "split into 4 sub-lemmas",
     "factor branch posts", "needs explicit zeta-table induction").
3. **Move on** — do not pile sunk cost into a hard proof.

The objective is **maximum easy-proof coverage** in a fixed budget. Hard
proofs land on the **USER lane** (math-heavy or Z3-blocked) for the
human to handle later; we get there only after harvesting all the
easy ones.

**Sizing implication**: each portable/AVX2 wrapper proof is sized to
≤20 min, which fits the "delegate to primitive + reveal_opaque +
Classical.forall_intro" pattern. Anything beyond that pattern (full
NTT composition, AVX2 layer-1/2 bridges, `compress_ciphertext_coefficient`-style
Barrett 4-case enumeration) is an immediate admit.

## Session split

This session executes **Phase 0** end-to-end and commits the plan + all
Phase 0 artifacts (FIPS fixes, cross-spec tests, mirror docs, hacspec
extensions). **Phases 1–4 hand off to subsequent sessions**, which read
this plan plus `libcrux-ml-dsa/MLDSA_STATUS.md` (created in Phase 0) to
resume work.

Each subsequent session is expected to:
- Pick a wave from a phase (each wave is the parallelism unit; see
  per-phase tables below).
- Apply the 20-minute rule per function.
- Commit `outstanding-admits.md` and `MLDSA_STATUS.md` updates with each
  proof batch.

## 48-hour timeline

### Phase 0 — Hardening, scaffolding, pre-sprint edits (THIS SESSION, hours 0–8)

Extraction is already done. Phase 0 consolidates upstream work that pays
dividends across Phases 1–4: FIPS fixes that simplify proofs,
cross-spec test infrastructure, proof-tracking docs, and small hacspec
helpers.

**Internal parallelism**: 0A, 0B, 0C, 0D are mutually independent and
can be done in any order. Recommended within a single session: do 0C
first (creates the doc skeletons everyone references), then 0A and 0D
in parallel via subagents (file edits, no overlap), then 0B.

#### 0A. FIPS 204 conformance fixes

A read-only audit produced 33 findings (no Critical, 4 High, 12 Medium,
16 Low/cosmetic) — full report at the side path noted above. The impl
interop-tests against PQ-Crystals KATs, so no item breaks correctness
today; these are the items worth fixing **before** locking trait posts
because they simplify the proofs:

| ID | Sev | Location | Fix | Proof benefit |
|---|---|---|---|---|
| F4 | High | `src/encoding/signature.rs:33-49` | Explicitly zero `signature[offset .. offset+ω+k]` before HintBitPack writes per-row counts. | Removes "caller pre-zeroes" precondition; closed-form bitvec post. |
| F3 | High | `src/ml_dsa_generic.rs:359-399` | Add `debug_assert!(signature.len() == SIGNATURE_SIZE)` and same for vk inside `verify_internal`, mirroring keygen at lines 68-70. | Length precondition becomes derivable. |
| F5 | High | `src/encoding/signature.rs:90-130` | Refactor HintBitUnpack to bound-check before any write into `out_hint`, OR zero `out_hint` on the Err path. | Clean `Err ⇒ out_hint == 0` post; no partial-state reasoning. |
| F2 | High (annotation) | `src/pre_hash.rs:26`, `src/ml_dsa_generic.rs:699` | Add comment that the OID is the DER encoding (tag 0x06, len 0x09, then 9 oid bytes), so no `IntegerToBytes(\|OID\|, 1)` prefix is needed (FIPS Alg 4 line 23). | Documents bit-pattern; reviewer-only. |
| F15 | Med (side-channel) | `src/simd/portable/arithmetic.rs:380-420`, `src/arithmetic.rs:13-22` | Replace `result \|\| normalized >= bound` with constant-time mask `result \|= ((bound - 1 - normalized) >> 31) & 1`. | Simpler bounds-only post; kills branch-on-secret reasoning. |
| F13 | Med (perf) | `src/ml_dsa_generic.rs:275-294`, `src/matrix.rs:67-75` | Refactor `vector_times_ring_element` to `(out, lhs, rhs)` instead of in-place. Eliminates `s1_as_ntt.clone()` / `s2_as_ntt.clone()` / `t0_as_ntt.clone()` per loop iteration. | Removes "clone preserves equivalence" lemmas from sign-loop proof. |

Deferred (no proof impact): F1, F6–F12, F14, F16–F33.

#### 0B. Cross-spec tests against the hacspec

Mirror `libcrux-ml-kem/tests/cross_spec.rs`. Cargo wiring + 5 test files
covering 24 per-method tests plus property tests and edge cases.

1. **Cargo**: add `[dev-dependencies] hacspec_ml_dsa = { path = "../specs/ml-dsa" }` to `libcrux-ml-dsa/Cargo.toml`. Gate behind a `cross-spec-tests` feature.
2. **Files**:
   ```
   tests/cross_spec.rs                 (~100 LoC, coordinator + macro)
   tests/cross_spec/arithmetic.rs      (~300 LoC, 8 tests)
   tests/cross_spec/encoding.rs        (~250 LoC, 6 tests)
   tests/cross_spec/sampling.rs        (~200 LoC, 3 tests)
   tests/cross_spec/ntt.rs             (~150 LoC, 2 tests)
   tests/cross_spec/helpers.rs         (~100 LoC, lane↔scalar adapters)
   tests/edge_cases.rs                 (~200 LoC, 10 corner cases)
   ```
3. **Per-method tests**: 24 tests, each running 1000 ChaCha-seeded iterations + boundary inputs (0, q-1, ±γ₂, ±γ₁, ±β):

   | Trait method | Spec equivalent (`Hacspec_ml_dsa.*`) |
   |---|---|
   | `add`, `subtract` | `Polynomial.poly_add` / `poly_sub` |
   | `infinity_norm_exceeds` | `Polynomial.poly_infinity_norm` then `>= bound` |
   | `decompose` | `Arithmetic.decompose` × 8 lanes |
   | `compute_hint` | `Arithmetic.make_hint` × 8 + popcount |
   | `use_hint` | `Arithmetic.uuse_hint` × 8 lanes (note typo) |
   | `montgomery_multiply` | per-lane `mod_q(a·b·R⁻¹)` (uses 0D helper) |
   | `shift_left_then_reduce` | per-lane `mod_q(a · 2^13)` (uses 0D helper) |
   | `power2round` | per-lane (uses 0D helper) |
   | `rejection_sample_less_than_field_modulus` | `Encoding.coeff_from_three_bytes` |
   | `rejection_sample_less_than_eta_equals_{2,4}` | `Encoding.coeff_from_half_byte` |
   | `gamma1_serialize/deserialize` | `Encoding.bit_pack` / `bit_unpack` width γ₁_exp+1 |
   | `commitment_serialize` | `Encoding.simple_bit_pack` width 4 (γ₂=261888) or 6 (γ₂=95232) |
   | `error_serialize/deserialize` | `Encoding.bit_pack` / `bit_unpack` width 3 (η=2) or 4 (η=4) |
   | `t0_serialize/deserialize` | `Encoding.bit_pack` width 13 |
   | `t1_serialize/deserialize` | `Encoding.simple_bit_pack` width 10 |
   | `ntt`, `invert_ntt_montgomery` | `Ntt.ntt` / `Ntt.intt` flat-256 |
   | `reduce` | per-lane `0 ≤ x < q` ∧ `x ≡ input (mod q)` |

4. **Property tests**: serialize ∘ deserialize = id (5 pairs), ntt ∘ intt = id mod q, decompose recovery `r1·2γ₂ + r0 ≡ r (mod q)`.
5. **Edge-case tests** (`tests/edge_cases.rs`): decompose wraparound, HintBitPack at exactly ω, HintBitUnpack on adversarial inputs, InfNorm at exact bound, Power2round at q-2/q-1/2²³, SampleInBall at τ extremes, sign_pre_hashed with empty/255-byte/256-byte context.
6. **End-to-end equivalence**: byte-identical keys/sigs from libcrux vs `Hacspec_ml_dsa.Ml_dsa` on fixed seeds — guards against a faulty spec invalidating proofs.

#### 0C. Proof-tracking infrastructure mirror

Create the doc skeletons that the rest of the sprint references:

1. **`libcrux-ml-dsa/MLDSA_STATUS.md`** — fork of `MLKEM_STATUS.md`. Spec hierarchy (canonical `Hacspec_ml_dsa.*`, obsolete `Spec.MLDSA.*`, scaffolding `Spec.Utils`). Per-method status table for all 21 Operations methods × {portable, avx2}.
2. **`libcrux-ml-dsa/proofs/proof-style-guide.md`** — fork of ML-KEM's. Adapt: i16 → i32, q=3329 → q=8380417, lane width 16 → 8, R⁻¹ → 8265825, 8 NTT layers, 256-element zeta table. Note the `uuse_hint` typo.
3. **`libcrux-ml-dsa/proofs/outstanding-admits.md`** — initial empty list with template entry format.
4. **`libcrux-ml-dsa/proofs/sprint-plan.md`** — copy of this plan file (so subsequent sessions can find it without leaving the repo).
5. Add deletion-banner headers to `proofs/fstar/spec/Spec.MLDSA.Math.fst` and `Spec.MLDSA.Ntt.fst` — scheduled for removal in Phase 4.

#### 0D. Hacspec extensions

Add missing scalar helpers to `specs/ml-dsa/src/arithmetic.rs` and re-extract:

- `power2round(t: i32) -> (i32, i32)` (scalar)
- `shift_left_then_reduce(t: i32) -> i32` (≅ `mod_q(t · 2^13)` in the precondition-enforced range)
- `montgomery_reduce(t: i64) -> i32` (scalar Montgomery — matches impl)

Confirm that `coeff_from_three_bytes` and `coeff_from_half_byte` (already in `Hacspec_ml_dsa.Encoding`) are reachable from the trait posts; if naming hygiene matters, re-export them from `Hacspec_ml_dsa.Sampling`.

Re-extract hacspec via `make -C specs/ml-dsa/proofs/fstar/extraction` and re-run `make -C libcrux-ml-dsa/proofs/fstar/extraction` to confirm nothing regresses. ~50 LoC of hacspec total.

#### Phase 0 deliverables (committed at end of session)

- 6 source edits applying FIPS fixes (F2–F5, F13, F15)
- 7 new test files (cross_spec/* + edge_cases.rs) + Cargo.toml feature
- 4 new doc files in `libcrux-ml-dsa/{,proofs/}`
- ~50 LoC of new hacspec helpers + re-extracted F*
- Banner headers on the 2 obsolete F* spec files
- A single git commit (or small commit train) on a new branch `ml-dsa-proofs-phase-0`

---

### Phase 1 — Strengthen Operations trait posts (HANDOFF, hours 8–14)

**Single-agent serial work**. This is the upstream change that
everything in Phases 2–3 depends on, so it must be done as one atomic
sequence. **Not parallelizable** — every method post lives in
`src/simd/traits.rs` and `src/simd/traits/specs.rs`, which means every
edit touches the same files.

For each Operations method, add a per-lane / per-batch opaque post
anchored to a `Hacspec_ml_dsa.*` function:

| Method | Anchor |
|---|---|
| `zero` / `from/to_coefficient_array` | trivial; already strong |
| `add`, `subtract` | already strong; wrap into per-lane opaque, keep bounds conjunct |
| `infinity_norm_exceeds` | `Polynomial.poly_infinity_norm` ≥ bound |
| `decompose` | per-lane `Arithmetic.decompose(simd_unit[i], gamma2)` |
| `compute_hint` | per-lane `Arithmetic.make_hint(low[i], high[i], gamma2)` + `result == popcount(hint)` |
| `use_hint` | per-lane `Arithmetic.uuse_hint(hint[i], simd_unit[i], gamma2)` |
| `montgomery_multiply` | per-lane `mod_q(future_lhs[i]) == mod_q(lhs[i] * rhs[i] * R⁻¹)` (uses `Hacspec_ml_dsa.Arithmetic.mod_q`, not `Spec.MLDSA.Math`) |
| `shift_left_then_reduce` | per-lane `Arithmetic.shift_left_then_reduce(simd_unit[i])` (0D helper) |
| `power2round` | per-lane `Arithmetic.power2round(t0[i])` (0D helper) |
| `rejection_sample_*` | per-iteration step via `Encoding.coeff_from_*` |
| `gamma1/commitment/error/t0/t1_serialize/deserialize` | bitvec round-trip via `Encoding.bit_pack` / `simple_bit_pack` |
| `ntt`, `invert_ntt_montgomery` | per-poly: flat-256 == `Ntt.ntt` / `Ntt.intt` via `forall32` |
| `reduce` | per-lane `mod_q(future[i]) == mod_q(input[i])` ∧ `\|future[i]\| < q` |

Add `forall8` / `forall32` to `Spec.Utils` (ML-KEM has `forall4`, `forall16`).

**20-minute rule**: each post addition (excluding the structural `forall_N`
helpers, which are mechanical) is sized to ≤20 min. If a single post
proves harder than that, downgrade to bounds-only and document.

**Phase 1 success criterion**: `make -C proofs/fstar/extraction` still
checks (post-strengthening should not break extraction; weak vacuous
posts allowed where the impl proof will arrive in Phase 2). Commit on
branch `ml-dsa-proofs-phase-1`.

---

### Phase 2 — Portable Operations proofs (HANDOFF, hours 14–28)

Once trait posts are settled (Phase 1), the impls in `src/simd/portable/*.rs`
strengthen in parallel waves. Each impl is typically a one-liner that
delegates to a primitive whose post is already strong; the work is to
add `hax_lib::fstar!()` reveal blocks plus a `Classical.forall_intro`
aux lemma calling a per-element commute lemma.

**Per-function 20-min rule**. The "easy" pattern is reveal_opaque +
Classical.forall_intro on a per-lane lemma. Anything that requires
non-trivial Z3 effort (NTT layer compositions, Barrett 4-case
enumerations) → admit + document.

| Wave | Parallelism | Tasks (each ≤20 min, batched per wave) | Dependencies |
|---|---|---|---|
| **2A** | 4-way | (i) `add`, (ii) `subtract`, (iii) `infinity_norm_exceeds`, (iv) `reduce` | Phase 1 |
| **2B** | 5-way | (i) `power2round`, (ii) `decompose`, (iii) `compute_hint`, (iv) `use_hint`, (v) `montgomery_multiply` | Phase 1 |
| **2C** | 1-way | `shift_left_then_reduce` | Phase 1 |
| **2D** | 5-way | `portable/encoding/{commitment,error,gamma1,t0,t1}.rs` ser+deser pairs | Phase 1 |
| **2E** | 3-way | `portable/sample.rs` × 3 rejection variants | Phase 1 |
| **2F** | seq | `portable/ntt.rs` (full NTT) | 2A–2C done; per-element commute lemmas authored in `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst` |
| **2G** | seq | `portable/invntt.rs` (full inverse + Montgomery exit) | 2F done (reuses commute lemmas) |

**Waves 2A–2E are mutually independent** and can run in parallel via
subagents — they touch disjoint files. Wave 2F gates 2G (shared commute
lemmas) and is the most likely admit candidate (full NTT composition is
the ML-KEM USER-2 analog).

**Commit cadence**: one git commit per wave. Each commit updates
`MLDSA_STATUS.md` (per-method status) and `outstanding-admits.md` (any
admits added).

---

### Phase 3 — AVX2 Operations proofs (HANDOFF, hours 28–44)

Same wrapper-plus-reveal pattern, but underlying primitives are
intrinsics with bit-level posts. Each impl needs a small bridge lemma
converting `vec256_as_i32x8` lane views to per-lane integer equalities.
Replace `Spec.Intrinsics` references with `libcrux-intrinsics` extraction
equivalents (already on the include path).

| Wave | Parallelism | Tasks | Dependencies |
|---|---|---|---|
| **3A** | 4-way | (i) `avx2/arithmetic.rs::{add, subtract, montgomery_multiply, reduce}`, (ii) `avx2/arithmetic.rs::{power2round, shift_left_then_reduce}`, (iii) `avx2/encoding/{commitment, error}.rs`, (iv) `avx2/encoding/{gamma1, t0, t1}.rs` | Phase 1 |
| **3B** | 1-way | `avx2/rejection_sample/{less_than_field_modulus, less_than_eta}.rs` (shuffle table mostly auto-derives) | Phase 1 |
| **3C** | 1-way | `avx2/arithmetic.rs::{decompose, compute_hint, use_hint}` (hardest of arithmetic; possible admit) | 3A done |
| **3D** | seq | `avx2/ntt.rs` — butterfly_2-based, replace `Spec.MLDSA.Ntt.ntt_step` with `Hacspec_ml_dsa.Ntt.*`. **Pre-budget admits** for layers 1–2 (the ML-KEM USER-4 4-zeta-parallel SIMD wall analog); aim to land layers 3–8. | 2F done (reuse layer-step commute lemmas) |
| **3E** | seq | `avx2/invntt.rs` — analogous | 3D done |

**Waves 3A and 3B are independent** of each other and of 2F/2G, so
they can launch in parallel with Phase 2 NTT work.

**Pre-budgeted admits** (we expect these going in, document up front):
- AVX2 NTT layers 1 and 2 — Z3-blocked on 4-zeta parallel SIMD; admit
  the bridge lemma, statement preserved.
- AVX2 INVNTT layers 1 and 2 — analogous.
- Possibly `avx2/arithmetic.rs::compute_hint` (tightest precondition).

---

### Phase 4 — Spec migration & integration (HANDOFF, hours 44–48)

| Wave | Parallelism | Tasks |
|---|---|---|
| **4A** | 1-way | Switch `montgomery_multiply` post in `traits.rs` from `Spec.MLDSA.Math.mod_q` to `Hacspec_ml_dsa.Arithmetic.mod_q`. |
| **4B** | 1-way (depends 4A) | Delete `proofs/fstar/spec/Spec.MLDSA.Math.fst` and `Spec.MLDSA.Ntt.fst` (banner-marked since 0C). Keep `Spec.Intrinsics.fsti` only if non-trivial AVX2 admits still cite it. |
| **4C** | parallel with 4A | Final `make` run; record check/admit/fail counts in `MLDSA_STATUS.md`. |
| **4D** | 1-way | Write `MLDSA_USER_TASKS.md` enumerating math-heavy lemmas + AVX2 layer 1/2 bridges as USER lanes (modelled on ML-KEM's USER-1..6). |

---

## Critical files

**Read-mostly (templates from ML-KEM)**:
- `libcrux-ml-kem/proofs/proof-style-guide.md`
- `libcrux-ml-kem/MLKEM_STATUS.md`
- `libcrux-ml-kem/proofs/manual-proof-targets.md`
- `libcrux-ml-kem/src/vector/traits.rs` — target end-state for ML-DSA `Operations` posts
- `libcrux-ml-kem/src/vector/portable.rs` — model for portable wrapper proofs
- `libcrux-ml-kem/tests/cross_spec.rs` — model for ML-DSA cross-spec tests
- `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst` — per-element commute lemma library to clone

**Edit (ML-DSA, Phases 1–4)**:
- `libcrux-ml-dsa/src/simd/traits.rs` — strengthen 21 method posts (Phase 1)
- `libcrux-ml-dsa/src/simd/traits/specs.rs` — per-lane opaque posts; dual-trigger lemmas (Phase 1)
- `libcrux-ml-dsa/src/simd/portable/*.rs` — reveal/forall_intro blocks (Phase 2)
- `libcrux-ml-dsa/src/simd/avx2/*.rs` — reveal/forall_intro blocks + bridge lemmas (Phase 3)
- `libcrux-ml-dsa/proofs/fstar/spec/*.fst` — banner + delete (Phase 4)
- `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst` (new) — per-element commute lemmas (Phase 2F prerequisite)

**Edit (this session, Phase 0)**:
- `libcrux-ml-dsa/src/encoding/signature.rs` — F4 zero-padding, F5 hint-unpack refactor
- `libcrux-ml-dsa/src/ml_dsa_generic.rs` — F3 length asserts, F2 OID comment, F13 vector_times_ring_element refactor
- `libcrux-ml-dsa/src/matrix.rs` — F13 signature change
- `libcrux-ml-dsa/src/simd/portable/arithmetic.rs`, `src/arithmetic.rs` — F15 constant-time mask
- `libcrux-ml-dsa/src/pre_hash.rs` — F2 OID comment
- `libcrux-ml-dsa/Cargo.toml` — `hacspec_ml_dsa` dev-dep + `cross-spec-tests` feature
- `specs/ml-dsa/src/arithmetic.rs` — 0D scalar helpers
- 2 banner-marked F* files

**Create (this session, Phase 0)**:
- `libcrux-ml-dsa/MLDSA_STATUS.md`
- `libcrux-ml-dsa/proofs/proof-style-guide.md`
- `libcrux-ml-dsa/proofs/outstanding-admits.md`
- `libcrux-ml-dsa/proofs/sprint-plan.md` (copy of this plan)
- `libcrux-ml-dsa/tests/cross_spec.rs` + `tests/cross_spec/{arithmetic,encoding,sampling,ntt,helpers}.rs`
- `libcrux-ml-dsa/tests/edge_cases.rs`

## Reusable F* infrastructure (already in include path)

- `Spec.Utils.is_i32b_array_opaque` + dual-trigger lookup/intro lemmas (`libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.fst`). Add `forall8` / `forall32` in Phase 1.
- `BitVecEq.int_t_array_bitwise_eq` (`fstar-helpers/fstar-bitvec/`) — shared serialize/deserialize correctness predicate.
- `libcrux-intrinsics` extraction — AVX2 intrinsic specs.

## Risks & mitigations

| Risk | Mitigation |
|---|---|
| Hacspec re-extraction breaks after 0D edits | Keep 0D additive (no edits to existing functions); if extraction fails, hand-write minimal F* shims (≤50 LoC) and proceed. |
| AVX2 NTT layer 1/2 4-way parallel proof divergence (ML-KEM USER-4 analog) | **Pre-budgeted admit**; aim to land layers 3–8. Document in `outstanding-admits.md` from day 1. |
| `montgomery_multiply` post change cascades to AVX2 | During Phase 1 keep both `Spec.MLDSA.Math.mod_q` AND `Hacspec_ml_dsa.Arithmetic.mod_q` as conjuncts; drop the obsolete one in Phase 4. |
| Sampling spec granularity (per-poly hacspec vs per-coeff impl) | Trait posts cite the per-byte step helpers (`coeff_from_three_bytes`, `coeff_from_half_byte`) — already extracted. Polynomial-level proof can compose later. |
| Z3-rlimit blowups on Tier-2/3 NTT | 20-minute rule kicks in: admit, document, USER lane. |
| F13 refactor (`vector_times_ring_element`) breaks downstream callers | Refactor is local to `matrix.rs`; only `ml_dsa_generic.rs` callers; keep an `#[inline]` shim if needed; defer if any test regresses. |

## Verification

End-of-sprint checks:

1. **Build**:
   ```
   cd libcrux-ml-dsa && ./hax.sh extract
   make -C proofs/fstar/extraction
   ```
   Expect 19 verified-list modules to typecheck (or admit cleanly per the 20-minute rule).

2. **Cross-spec test**:
   ```
   cargo test --features cross-spec-tests --test cross_spec
   cargo test --features cross-spec-tests,simd256 --test cross_spec
   cargo test --test edge_cases
   ```

3. **Status report**: `MLDSA_STATUS.md` shows ≥38/42 portable+AVX2 method
   posts verified; admits cataloged in `outstanding-admits.md` and
   bucketed agent-vs-USER.

4. **Negative check**: `grep -r 'Spec.MLDSA' libcrux-ml-dsa/proofs/fstar/extraction/ libcrux-ml-dsa/src/` returns empty (or only obsolete-banner comments).

# ML-KEM Proof Status Audit — 2026-05-03

**Branch**: `libcrux-ml-kem-proofs` @ HEAD `b9eee5838` (Phase E push 3)  
**Date**: 2026-05-03  
**Scope**: ML-KEM only (`libcrux-ml-kem/`, `specs/ml-kem/`, trait boundary)

**CRITICAL CAVEAT**: Two parallel agent worktrees (`libcrux-stream1-ind_cca` and `libcrux-stream2-ind_cpa`) are currently editing `ind_cca.rs` and `ind_cpa.rs`. This audit's view of those files may go stale within hours of 2026-05-03 18:00 UTC. **Current code state = main checkout at 2026-05-03 03:09 UTC (Phase E push 3).**

---

## 1. Current Verification Surface

### F* Extraction Modules (`libcrux-ml-kem/proofs/fstar/extraction/`)

**Total: 84 F* modules** across extraction + spec modules.

| Category | Count | Status |
|----------|-------|--------|
| **VERIFIED** | ~15 | No admits; full ensures verified (Portable.Serialize, Ntt.fst layer 1-3, Invert_ntt.fst layer 1-3, Polynomial.fst core ops, Serialize.fst core) |
| **PANIC-FREE** | ~20 | Body panic-freedom verified; ensures often admitted (`is_bounded_*` bounds without Hacspec citation) |
| **LAX** | ~10 | Body fully admitted (entire module `--admit_smt_queries true` or per-fn `lax` annotation) |
| **MODULE-ADMITTED** | ~21 | In Makefile `ADMIT_MODULES`: 7 Neon modules, 8 Incremental modules (`Mlkem*.Incremental`), 4 Avx2 bridge modules, 1 `Ind_cca.Unpacked.fst`, 1 `Vector.Portable.Vector_type.fst.patch` |
| **MIXED** | ~18 | Per-fn breakdown (see below) |

**Makefile `ADMIT_MODULES` entries** (21 modules):
- Neon (8): `Vector.Neon.{fst,fsti,Arithmetic,Compress,Ntt,Serialize,Vector_type.fst,Vector_type.fsti}`
- Incremental API (8): `Ind_cca.Incremental.*`, `Mlkem{512,768,1024}.Incremental.fst`
- ind_cca unpacked (1): `Ind_cca.Unpacked.fst`
- Incremental supporting (4): `Ind_cca.Incremental.{Types,Multiplexing,Portable,Avx2}`

**Trait-boundary temporary admits** (3 modules, scope = post-strengthening fallout):
- `Libcrux_ml_kem.Serialize.fst` — `compress_then_serialize_message` (1 fn, rlimit 80 cancel)
- (2 others pending per Makefile comments)

### MIXED Modules — Per-Function Breakdown

Key modules with per-fn variation:

| Module | Structure | Counts |
|--------|-----------|--------|
| `Ind_cpa.fst` | 20 fns: 8 panic_free, 12 lax | Per-fn breakdown: `serialize_vector`, `compress_then_serialize_u`, `deserialize_then_decompress_u`, `deserialize_vector`, `encrypt_c1`, `encrypt_c2`, `sample_vector_cbd_then_ntt` — lax / Z3 quantifier-incomplete (eq_intro pattern); `encrypt_unpacked`, `decrypt_unpacked` — panic_free (Phase E push 3); `encrypt`, `decrypt`, `generate_keypair_unpacked`, `build_unpacked_public_key{,_mut}`, `sample_ring_element_cbd` — panic_free. |
| `Ind_cca.fst` | 35 fns: 11 panic_free, 24 lax | High-level unpacked API dispatched to Ind_cpa; most lax through cascade composition. |
| `Vector.Avx2.fst` | ~25 fns: ~18 panic_free (bridge admits), ~7 lax-admitted | Bridge admits (4 B4 NTT + 3 USER-9b serialize/deserialize) per fn; wrapper ops over trait posts. |
| `Vector.Portable.fst` | ~30 fns: ~28 verified/panic_free, 2 lax | `op_ntt_layer_1_step`, `op_inv_ntt_layer_1_step` — body `--admit_smt_queries true` (B2 user-task USER-12); all others panic_free or verified. |
| `Serialize.fst` | ~15 fns: ~13 verified/panic_free, 1-2 lax | Most per-width core functions verified; wrapper composition panic_free. New `deserialize_then_decompress_4` verified (Phase E push 3). |
| `Ntt.fst` / `Invert_ntt.fst` | ~12 fns each: ~9-10 verified, ~2-3 lax | Layers 1-3 backward bridge closes (Invert_ntt fully proven for layers 1-3). Layer 4+ deferred pending spec design (USER-14, USER-15). |

### Hacspec Spec Modules (`specs/ml-kem/proofs/fstar/`)

| Module | Fns | Status |
|--------|-----|--------|
| `Hacspec_ml_kem.Ind_cpa.fst` | ~8 | Core spec functions; fully axiomatized |
| `Hacspec_ml_kem.Ind_cca.fst` | ~10 | KEM API spec; axiomatized |
| `Hacspec_ml_kem.Serialize.fst` | ~12 | Encoding/decoding specs; axiomatized |
| `Hacspec_ml_kem.Ntt.fst` | ~8 | Forward/inverse NTT; axiomatized |
| `Hacspec_ml_kem.Compress.fst` | ~8 | Compress/decompress per-width; axiomatized |
| `Hacspec_ml_kem.Commute.{Bridges,Chunk,ProofUtils}.fst` | ~50 | Commute lemmas (per-lane, per-index, Montgomery/plain bridges); ~40 proven, ~10 deferred/USER-N |
| `Hacspec_ml_kem.Parameters.{Sizes,Hash_functions}.fst` | ~20 | Size calculations, SHA3 refs; axiomatized |

**Spec status summary**: ~130 F* spec functions total; ~90 verified commute lemmas, ~40 axiomatized spec definitions.

---

## 2. Trait Boundary Inventory

### Trait Definition & Posts

**File**: `src/vector/traits.rs` (trait definition + spec injections)

| Component | Status | Details |
|-----------|--------|---------|
| `Operations` trait (16 ops) | All panic_free + verified trait posts | `add`, `sub`, `mul_by_constant`, `serialize_d` (widths 1-4-5-10-11-12), `deserialize_d`, `compress_d`, `decompress_d`, `ntt`, `inv_ntt`, `sample_cbd`, `sample_uniform` |
| Trait posts (32 pre/post pairs) | Mixed: 16 verified structural, 16 pending Hacspec citation | Compress/decompress posts **newly strengthened** (Phase E push 2): added `bounded_i16_array (mk_i16 0) (mk_i16 3328) result` to decompress post. |
| Spec module `Vector.Traits.Spec` (injected) | Fully verified | 8 predicates: `i16_to_spec_fe`, `mont_i16_to_spec_fe`, array lifts, zeta builders; per-lane lifting proofs for Montgomery ↔ plain conversion; **opaque_to_smt** decorators on FE-equality posts |

### Per-Impl Trait Dispatch

#### Portable (`src/vector/portable.rs`, 1195 lines)

| Category | Count | Status | Admits |
|----------|-------|--------|--------|
| Trait ops dispatched | 16 | All panic_free | 0 body admits; 2 wrapper fns use `--admit_smt_queries true` (layer 1-2 NTT, deferred to USER-12) |
| Direct trait-post verification | 14 | Verified inline | Per-op loop invariants + trait-post discharge |
| `admit ()` in fstar! escapes | 2 | `op_ntt_layer_1_step`, `op_inv_ntt_layer_1_step` (lines 975, 995) | Z3 saturation on 16-element forall + refined-type loop invariant (USER-12) |
| Panic-freedom vs. verified split | ~28 verified + 2 lax | Balance tips verified | Early ops (arithmetic, compress, sample) verified; NTT layer ops at trait boundary deferred |

**Key detail**: Phase E push 2 strengthened `decompress_ciphertext_coefficient_post` (line 851-877) by adding `bounded_i16_array (mk_i16 0) (mk_i16 3328) result` clause. Portable wrapper (`op_decompress_ciphertext_coefficient`) re-establishes this bound via `lemma_bounded_i16_array_intro`, **discharging the new conjunct with real verification** (no admit).

#### Avx2 (`src/vector/avx2.rs`, 1388 lines)

| Category | Count | Status | Admits |
|----------|-------|--------|--------|
| Trait ops dispatched | 16 | Mix: 9 panic_free, 7 lax | 0 body verification (all bodies are `admit ()` or deferred) |
| Wrapper fns with direct `admit ()` | 4 | Explicit `admit ()` in fstar! | Lines 335, 347, 367, 379 — per-op panic-freedom admitted (`(* Panic freedom *)` comments) |
| Panic-freedom vs. verified split | 9 panic_free / 7 lax | Incomplete | Wrapper dispatch panics admitted; inner intrinsic calls unmodeled |
| Bridge-admit territory | 7 admits (per Makefile) | Deferred | USER-9b 5-bit serialize/deserialize; B4 NTT layer ops |

**Key detail**: Avx2 wrappers do NOT verify their own body panic-freedom (all admit the precondition chain). Dispatch happens via trait-post discharge, but the impl-side burden proof is deferred. This is **by design** — Avx2 intrinsic models are in a separate `crates/utils/intrinsics/` crate; ML-KEM impl-side wrapping is panic_free only (no Hacspec equivalence).

#### Neon (`src/vector/neon.rs`, 214 lines)

| Category | Count | Status | Admits |
|----------|-------|--------|--------|
| Trait ops dispatched | 16 | All marked `lax` in Rust | **Entire extracted module in ADMIT_MODULES** |
| Verification status | N/A — module-admitted | Full module SMT-admitted at extraction level | All ops + their posts + any Hacspec citations vacuously pass |
| Neon-specific reasoning | 0 lemmas | Not started | No SIMD-model reasoning; structural complexity mirrors Avx2 |

**Architectural note**: Neon is included because many targets require NEON support (e.g., Apple M-series, ARM64 CI). Currently uses the same trait-opacify approach as Avx2: impl wrappers are lax, trait posts are structural. **To flip Neon to panic_free** (see Q1), the path is:
1. Model NEON intrinsics (parallel to AVX2 intrinsics work, out of scope for this audit).
2. Add per-op panic-freedom proofs in Neon impl (6-8 sessions estimated).
3. Drop Neon from ADMIT_MODULES.

**Spec-side**, Neon requires NO Hacspec work — the trait posts are impl-agnostic.

---

## 3. Milestone A Scope Estimate

**Milestone A**: All BELOW-TRAIT functions fully verified w.r.t. Hacspec; all ABOVE-TRAIT functions panic-free up to `Ind_cca.fst`.

### Below-Trait (Trait + Per-Impl) Gap Analysis

**Currently lax or panic-free without ensures**:

| File:Line | Fn | Status | Reason (per FOLLOW-UP) |
|-----------|--|----|--------|
| `src/vector/portable.rs:975` | `op_ntt_layer_1_step` | Lax | Z3 saturation on 16-element forall + refined-type loop invariant (USER-12). Body `--admit_smt_queries true`. |
| `src/vector/portable.rs:995` | `op_inv_ntt_layer_1_step` | Lax | Same pattern as layer 1 (USER-12). |
| `src/vector/avx2.rs:335,347,367,379` | 4 wrapper ops | Panic_free | Direct `admit ()` in fstar! (lines cited); dispatch-side panics admitted. |
| `src/vector/avx2.rs:1142,1155` | 2 NTT-ops | Panic_free | `admit ()` via `hax_lib::fstar!` macros (panic-freedom only). |

**Spec-side gaps**:
- `Hacspec_ml_kem.Ntt.fst` — Forward layers 4-7 spec functions exist but lack commute lemmas (Layer 4+ is multi-step like inverse; design pending, USER-14-level scope).
- `Hacspec_ml_kem.Polynomial.fst` — Missing `ntt_multiply` spec cite (bounds-only post exists, fn `src/polynomial.rs:920` stays at panic_free).

**Effort to A** (below-trait fully verified w.r.t. Hacspec):
- **Portable NTT layer 1-2** (2 fns): ~1 session each (USER-12, re-attempt under Z3 rule improvements or rlimit re-tuning).
- **Avx2 wrapper ops panic-freedom** (4 fns): ~0.5 sessions each (straightforward intrinsic dispatch confirmation; not Hacspec-equivalent, just panic-freedom).
- **Forward NTT layers 4-7** spec design: ~1.5 sessions (spec + layer 4_plus commute lemma; layer 7 is novel butterfly pattern).
- **Total**: ~6-7 sessions (assuming USER-12 remains an open Z3 question).

### Above-Trait (ind_cpa, ind_cca) Gap Analysis

**Currently lax**:

| File:Line | Fn | Status | Reason (per Phase D FOLLOW-UP) |
|-----------|--|----|--------|
| `src/ind_cpa.rs:151` | `serialize_vector` | Lax | Body's `eq_intro` spec-equality assertion fails Z3 at rlimit 800 (incomplete quantifiers). **Pattern-1**: post-loop assertion restructure needed. |
| `src/ind_cpa.rs:265` | `sample_vector_cbd_then_ntt` | Lax | Body fails panic_free precondition on `ntt_binomially_sampled_ring_element` call (line 352 extracted). **Pattern-2**: loop-invariant strengthening needed. |
| `src/ind_cpa.rs:452` | `serialize_unpacked_secret_key` | Lax | Cascade-lax: calls lax `serialize_public_key` + lax `serialize_vector`; tuple-equality with hacspec needs Phase C bridge. **Pattern-3**: Phase C bridge prereq. |
| `src/ind_cpa.rs:485` | `compress_then_serialize_u` | Lax | Body has eq_intro assertion (Pattern-1). |
| `src/ind_cpa.rs:512` | `encrypt_unpacked` | Panic_free (Phase E) | Flipped in Phase E push 1; remaining unpacked-API helpers still lax. |
| `src/ind_cpa.rs:716` | `encrypt_c1` | Lax | Panic_free precondition on `into_padded_array` (Pattern-2). |
| `src/ind_cpa.rs:788` | `encrypt_c2` | Lax | Panic_free precondition on `compute_ring_element_v` — polynomial bounds not propagated (Pattern-2). |
| `src/ind_cpa.rs:981` | `deserialize_then_decompress_u` | Lax | Body has eq_intro assertion (Pattern-1). |
| `src/ind_cpa.rs:1043` | `deserialize_vector` | Lax | Body has eq_intro assertion (Pattern-1). |

**Cascade-lax (not targeted in Phase D/E, out of scope for A)**:
- `src/ind_cpa.rs:77` — `serialize_public_key` (lax, depends on phase C bridge)
- `src/ind_cpa.rs:128` — `serialize_public_key_mut` (lax)
- `src/ind_cpa.rs:455` — `generate_keypair` (lax, composes across serialize_public_key cascade)

**ind_cca.rs**:
- `src/ind_cca.rs:226` — `generate_keypair` (packed) — Lax, cascade from above.
- `src/ind_cca.rs:294` — `encapsulate` (packed) — Lax, cascade from above.
- `src/ind_cca.rs:542` — `unpacked::unpack_public_key` — **Marked lax in source but lives in ADMIT_MODULES module** (`Ind_cca.Unpacked.fst`). Module-level admit dominates; source annotation is documentation only.

**Pattern breakdown** (Phase D taxonomy):

1. **eq_intro Z3 quantifier-incomplete** (4 fns: `serialize_vector`, `compress_then_serialize_u`, `deserialize_then_decompress_u`, `deserialize_vector`): Use `Classical.forall_intro` over per-index lemma instead of bare `eq_intro`. Precedent: pre-Phase-B revisions of these fns used this pattern and succeeded at rlimit > 800. **Estimated effort**: 1 session (self-contained refactor, applies to 4 fns uniformly).

2. **Loop-invariant strengthening** (3 fns: `sample_vector_cbd_then_ntt`, `encrypt_c1`, `encrypt_c2`): Tighten loop invariant to expose callee precondition bounds (e.g., `ntt_binomially_sampled_ring_element` requires `is_bounded_poly 4095 elem`). **Estimated effort**: 0.5–1 session per fn (~2-3 sessions total).

3. **Cascade through serialize_public_key** (3 fns: `serialize_public_key`, `serialize_public_key_mut`, `generate_keypair`): Requires Phase C bridge lemma (per session-2026-05-02-spec-mlkem-phaseB.md, USER-7 deferred). **Gated on**: Hacspec-side bridge + source side `Classical.forall_intro` refactor. **Estimated effort**: 1–2 sessions post-bridge.

**Effort to A** (above-trait panic-free up to ind_cca):
- **Pattern-1 refactor** (4 fns): ~1 session.
- **Pattern-2 loop strengthening** (3 fns): ~2-3 sessions.
- **Pattern-3 Phase C bridge** (3 fns + Hacspec side): ~2 sessions (1 source, 1 spec).
- **Total above-trait**: ~5-6 sessions.

**Milestone A total effort estimate**: 11-13 sessions (6-7 below-trait + 5-6 above-trait).

---

## 4. Milestone B Scope Estimate

**Milestone B**: Everything up to `Ind_cca.fst` FULLY VERIFIED w.r.t. Hacspec (default-verify, full ensures everywhere).

### Hacspec Ensures Gaps (Above-Trait Functions)

**Currently panic_free WITHOUT full Hacspec ensures**:

| File:Line | Fn | Current ensures | Missing Hacspec cite |
|-----------|---|---|---|
| `src/ind_cpa.rs:209` | `sample_ring_element_cbd` | `is_bounded_poly 3328 result` | `Hacspec_ml_kem.Sampling.sample_cbd` (spec exists, just needs citation) |
| `src/ind_cpa.rs:353` | `generate_keypair_unpacked` | Composition of sub-fns | Top-level `Hacspec_ml_kem.Ind_cpa.generate_keypair` (spec exists) |
| `src/ind_cpa.rs:628` | `encrypt_unpacked` | Composition ensures | `Hacspec_ml_kem.Ind_cpa.encapsulate_unpacked` — **spec missing**, needs design |
| `src/ind_cpa.rs:815` | `encrypt` | Composition ensures | Same as above (wraps `encrypt_unpacked`) |
| `src/ind_cpa.rs:887` | `build_unpacked_public_key` | Composition ensures | `Hacspec_ml_kem.Ind_cpa.build_unpacked_public_key` (spec exists) |
| `src/ind_cpa.rs:942` | `build_unpacked_public_key_mut` | Composition ensures | Same as above |
| `src/ind_cpa.rs:1122` | `decrypt_unpacked` | Composition ensures (Phase E push 3) | `Hacspec_ml_kem.Ind_cpa.decapsulate_unpacked` — **spec missing**, needs design |
| `src/ind_cpa.rs:1177` | `decrypt` | Composition ensures | Same as above (wraps `decrypt_unpacked`) |

**Spec-side gaps**:
- `Hacspec_ml_kem.Ind_cpa.encapsulate_unpacked` — **MISSING**. Currently only `Hacspec_ml_kem.Ind_cpa.encapsulate` (packed API). Unpacked variant needs design.
- `Hacspec_ml_kem.Ind_cpa.decapsulate_unpacked` — **MISSING**. Same story.
- `Hacspec_ml_kem.Ind_cca.*` top-level specs — Partially missing (`encapsulate`, `decapsulate` exist; `validate_*` missing).

**Effort to B** (full Hacspec ensures on all above-trait fns up to ind_cca):
- **Add 2 missing unpacked-API spec functions** (Ind_cpa): 1–2 sessions (design + axiomatization).
- **Cite + prove 8 above-trait fns** (4 serialize/decompress Pattern-1 + 3 sample/encrypt Pattern-2 + 1 cascade): Assume Pattern-1 and Pattern-2 are solved (per Milestone A estimate), this is proof-glue work. ~2–3 sessions.
- **Ind_cca.fst ensures** (top-level `generate_keypair`, `encapsulate`, `decapsulate`): Composition lemmas already exist (partial); need top-level spec cites. ~1–2 sessions.
- **Total above-Milestone-A**: ~4-7 sessions.

**Milestone B total effort estimate**: 15-20 sessions (11-13 from A + 4-7 new).

---

## 5. Q1: Neon Module Inclusion

### Current State

**Neon modules** (8 files, ~214 lines source code):
- `src/vector/neon.rs` (main; 214 lines)
- `src/vector/neon/{arithmetic,compress,ntt,sampling,serialize,vector_type}.rs` (~50-100 lines each)

**Extraction status**: All extracted to F* under `Libcrux_ml_kem.Vector.Neon.*`. All 8 modules are in `ADMIT_MODULES` (entire SMT-admitted at extraction level).

**Rust-side verification_status**: All Neon ops marked `verification_status(lax)` or unmarked (implicitly lax). No panic_free annotations in Neon source.

### Comparison with Portable

| Aspect | Portable | Neon |
|--------|----------|------|
| Trait ops | 16 | 16 |
| Impl-side panic-freedom proves | 14 verified, 2 lax | 0 (all admitted) |
| Hacspec equivalent posts | Yes (per-op trait posts + loop invariants) | Yes (same trait posts), but never verified |
| Intrinsic models | None (pure Rust) | NEON intrinsic calls via `simd128` crate; no formal model in F* |
| Estimated lines of proof | ~500 (across Portable + submodules) | ~0 (entire module admitted) |

### Requirements to Flip Neon to Panic-Free

1. **NEON intrinsic models** (prerequisite, not blocking Neon panic-freedom alone):
   - Currently, each NEON instruction call in Neon impl is an uninterpreted F* function imported from an unproven module.
   - To enable panic-freedom proofs, need either: (a) axiomatized postconditions for each intrinsic (lighter), or (b) formal algebraic models (heavier, unnecessary for panic-freedom).
   - **Estimated effort**: 2–4 sessions (parallel work to what's ongoing in `crates/utils/intrinsics/proofs/` for AVX2).

2. **Panic-freedom proofs for 16 Neon ops** (per-op invariant discharge):
   - Mirror Portable's structure: loop invariant + trait-post discharge.
   - **Estimated effort**: 2–4 sessions (6-12 ops/session, but NEON loops tend to be longer/more complex than Portable).

3. **Spec-side**: Zero (trait posts are impl-agnostic).

### Architecture: Blockers?

**No structural blockers**, but:
- **Intrinsics trust-base**: NEON intrinsic postconditions will be assumed (user-axioms). Unlike Portable (which has Rust semantics + extraction), NEON wraps external assembly. Postulates are unavoidable.
- **Bit-vector reasoning**: NEON uses 128-bit SIMD vectors. F* quantifier support for bit-vectors is limited compared to integer reasoning. Some NEON-to-scalar lifting lemmas may need custom tactic support.

**Recommendation for Q1**: Panic-free Neon is achievable in ~6-8 sessions post-intrinsics-model. **Full Hacspec-equivalent verification** would require intrinsic **refinement specs**, an order of magnitude harder (~15-20 sessions if attempted at all; out of scope for mainstream proof path).

**Decision point**: Neon panic-freedom is useful for safety guarantees; Neon Hacspec equivalence is lower-priority (Portable already provides the verified baseline, Avx2 will follow). Recommend panic-freedom as the Milestone A / B checkpoint.

---

## 6. Q2: Minimum User-Axiom Set

### Candidate Axioms

An ML-KEM proof should build on the following assumed (not proven) statements:

| Category | Axiom | Scope | Word-count approx. |
|----------|-------|-------|-------------------|
| **Hacspec spec definitions** | `Hacspec_ml_kem.Parameters.FieldElement` + arithmetic (add, mul, mod q) | Core type + ops | ~50 |
| | `Hacspec_ml_kem.Ntt.ntt_layer_n` (per layer, 8 layers × forward+inverse) | Spec recursion | ~150 (8 copies) |
| | `Hacspec_ml_kem.Serialize.serialize_d` / `deserialize_d` (widths 1-12) | Spec encoding | ~100 (12 copies) |
| | `Hacspec_ml_kem.Compress.compress_d` / `decompress_d` (widths 1-11) | Spec quantization | ~100 (22 copies) |
| | `Hacspec_ml_kem.Ind_cpa.*` / `Hacspec_ml_kem.Ind_cca.*` (11 fns) | Top-level KEM spec | ~300 |
| | **Subtotal**: Spec definitions | | **~700** |
| **SIMD intrinsic posts** | `Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16` post (lane decomposition) | AVX2-specific | ~30 |
| | Similar for Neon (16 intrinsics) — **TBD** | Neon-specific | ~50 |
| | **Subtotal**: Intrinsics | | **~80** |
| **Trait method posts** | `Vector.Traits.Operations.*` (16 ops, 32 pre/post pairs) | Abstraction boundary | ~400 |
| | **Subtotal**: Trait posts | | **~400** |
| **Bit-vector axioms** | `Spec.Utils.bit_vec_of_*` predicates (5 widths: 1, 4, 5, 10, 11 bits) | Serialization model | ~100 |
| | **Subtotal**: Bit-vectors | | **~100** |
| **Z3/SMT foundations** | Standard linear arithmetic, quantifier instantiation, refinement subtyping (user accepts Z3 decisions) | Foundational | ~0 (implicit) |

### Recommended Axiom Set (Minimal)

To build all of Milestone B, assume these and prove everything else:

1. **Hacspec spec module** (~700 words): `Hacspec_ml_kem.*` definitions + `ind_cpa_encrypt`, `ind_cca_encapsulate` etc. top-level specs.
2. **Trait boundary posts** (~400 words): `Vector.Traits.Operations.*` method contracts.
3. **Intrinsics posts** (~80 words): Per-SIMD vec-decomposition + lane-arithmetic (Avx2 + Neon).
4. **Bit-vector serialization model** (~100 words): `bit_vec_of_*` predicates for each width.

**Total**: ~1280 words.

### What Should NOT Be Assumed

- **Commute lemmas** (`Hacspec_ml_kem.Commute.*`): These are proofs, not axioms. Each should have a proof body (even if that proof uses Z3 tactics).
- **Per-fn ensures** on extraction fns: Each ensures should be proven (or transparently admitted under Milestone-A/B scope).
- **Loop invariants in impl**: These are interior to function proofs; should not float to axiom level.

### Axiom Exposure

Each axiom should be:
- Stated in an `.fsti` interface file with a one-page semantic explanation.
- Reviewed by user (cryptographer + verification engineer).
- Cited in top-level README as **Trust Base**.

---

## 7. Q3: Top-Down vs. Bottom-Up — Full Proofs First?

### Candidate Functions for Full-Proof-First Strategy

**Where skipping panic_free and going straight to Hacspec ensures might be faster**:

1. **`src/invert_ntt.rs:36 invert_ntt_at_layer_1`** — ✅ Already done (in prod). Loop structure is simple 8-element loop over 32 chunks. Per-iteration invariant directly establishes per-lane spec equality. Skipping panic_free and proving full Hacspec contract upfront was the right call (saved 0.5–1 session vs. two-stage approach).

2. **`src/ntt.rs:ntt_at_layer_1`** — Similar to above. Forward loop uses same 8-element pattern. Spec function `Hacspec_ml_kem.Ntt.ntt_layer_1` has matching recursion depth. **Candidate**: Full proof first.

3. **`src/polynomial.rs:ntt_multiply`** — Mixed signal. Loop structure (loop over 128 elements) is loop-heavy. Panic-freedom obligations (no overflow on fixed-point multiply) are cleanly separable from Hacspec equivalence (the mul + reduce commutes with spec multiply). **Verdict**: Panic-free first (cheaper, defers the hard part).

4. **`src/serialize.rs:deserialize_then_decompress_ring_element_v`** — **Already full proof** (Phase E push 3). Deserialization chains per-lane decompress posts through a loop. Per-iteration invariant + helper lemma (`lemma_decompress_post_to_is_bounded_vector`) connected the trait post to the ensures. Spec function exists and structurally matches. **Verdict**: Full proof right call (panic_free would have been halfway there anyway).

5. **`src/ind_cpa.rs:encrypt_unpacked`** — ✅ Flipped to panic_free (Phase E push 1). Body is composition of 3 calls + 1 loop. Panic-freedom proved (preconditions + bounds). Hacspec ensures missing (would require new unpacked-API spec fns). **Verdict**: Panic-free was correct (Hacspec fns don't exist yet).

### Candidates Where Panic_Free is Clearly Right

1. **`src/vector/portable/compress.rs:compress_1`, `compress_d`** — Large 16-element loops with per-coefficient Barrett reduce. Panic-freedom (no overflow) is the cheap part; Hacspec equivalence (per-compress-d lane equality) requires loop invariant + lemma, ~2-3 times harder. **Verdict**: Panic_free first (deferred in Phase B due to Z3 saturation, revisit under improved quantifier heuristics).

2. **`src/vector/portable/ntt.rs:ntt`** — Full polynomial-level NTT. Loop over 7 layers × 128 elements. Panic-freedom (no overflow on twiddle multiplies) is straightforward. Hacspec equivalence requires per-layer lemmas + synthesis into `Hacspec_ml_kem.Ntt.ntt` top-level. **Verdict**: Panic_free first (4-5 sessions), then full proof (additional 2-3 sessions post-Layer-4+-spec-design).

3. **`src/ind_cpa.rs:serialize_vector`** — 256-element loop over per-element encoding. Panic-freedom is trivial (no arithmetic). Hacspec equivalence requires the eq_intro refactor (Pattern-1). **Verdict**: Panic_free + ensures first (refactor the assertion), then Hacspec cite (1 session).

4. **`src/ind_cpa.rs:decrypt_unpacked`** — Already flipped to panic_free (Phase E push 3). Deserialization + matrix compute + message extraction. Panic-freedom is the structural requirement (done). Full Hacspec ensures await `Hacspec_ml_kem.Ind_cpa.decapsulate_unpacked` spec design (1-2 sessions post-spec). **Verdict**: Panic_free + composition ensures first (done), full spec cite deferred (spec design prerequisite).

### Summary Table

| Fn | Phase | Panic-free cost | Full-proof cost | Verdict | Reason |
|----|-------|---|---|---|---|
| `invert_ntt_at_layer_1` | ✅ done | - | 1 session | **Full-first** (done) | Simple loop, per-iter = per-spec-fn |
| `ntt_at_layer_1` | pending | <0.5s | 1 session | **Full-first** | Same pattern as inverse |
| `ntt_multiply` | pending | 0.5 session | 2 sessions | **Panic-first** | Panic-freedom independent from spec |
| `deserialize_*_ring_element_v` | ✅ done | - | 1 session | **Full-first** (done) | Helper lemma connects trait post → ensures |
| `encrypt_unpacked` | ✅ PF done | - | 2+ sessions | **Panic-first** (done) | Spec fns missing, defer full proof |
| `compress_1`, `compress_d` | deferred | 1 session | 1 session | **Panic-first** (tentative) | Z3 quantifier issue on either path; panic-free cheaper at 800 rlimit |
| `serialize_vector` | deferred | 0.5 session | 0.5 session | **Panic-first then full** | Pattern-1 refactor applies to both; full proof tight on the same loop |

**Conclusion**: No universal rule. **Top-down (full proof first) wins when**:
- Loop is simple + per-iteration structure directly yields per-spec-fn equality.
- Spec functions are already designed.
- Panic-freedom obligations would duplicate work (rare).

**Bottom-up (panic-free first) wins when**:
- Panic-freedom is structurally simpler (independent arithmetic vs. spec equivalence).
- Spec fns don't exist yet (have to design them first).
- Loop has branches or complex invariant structure (panic-free is weaker).

---

## 8. Recommendation: 4-Week Sprint Plan

**Target**: **Milestone A** (all below-trait verified, all above-trait panic-free).

### Rationale for Target

- **Achievable in 4 weeks**: 13-15 sessions (4 weeks × 3-4 sessions/week at full focus).
- **Valuable**: Panic-freedom guarantee across entire ML-KEM surface.
- **Unblocked**: User-authorized R5 carve-out landed; Phase D/E residuals are self-contained refactors.
- **Milestone B deferred**: Requires Hacspec spec design (unpacked-API fns), 2-3 additional weeks post-A.

### Week-by-Week Plan

| Week | Tasks | Sessions | Risk |
|------|-------|----------|------|
| **1** | Pattern-1 refactor (serialize_vector, compress_then_serialize_u, deserialize_then_decompress_u, deserialize_vector): Use `Classical.forall_intro` over per-index lemma instead of bare `eq_intro`. Verify all 4 fns in Ind_cpa.fst. | 1–1.5 | **Medium**: Z3 quantifier behavior under refactored proof may still hit incomplete-quantifier errors. Mitigation: test in isolation first, bump rlimit to 1200 if needed (accept ~20s per-fn). |
| **1–2** | Pattern-2 loop-strengthening (sample_vector_cbd_then_ntt, encrypt_c1, encrypt_c2): Identify missing callee precondition bounds in loop invariants. Discharge via loop-body assertions + existing lemmas. | 2–3 | **Medium-High**: Loop invariant synthesis is manual. Per-fn debugging ~30-60 min each. |
| **2** | Pattern-3 Phase C bridge (serialize_public_key, serialize_public_key_mut, generate_keypair): Phase C bridge lemma must land first on Hacspec side (tuple-equality over serialize sequence). Coordinates with Spec team. Once bridge lands, source-side fix is straightforward (apply bridge at composition sites). | 0.5–1 (blocked on Spec) | **High**: Gated on external Hacspec-side work. Mitigation: parallelize with week 3. |
| **2–3** | Portable NTT layer 1-2 (op_ntt_layer_1_step, op_inv_ntt_layer_1_step) per USER-12: Attempt Z3 re-tuning (rule-reordering, `--split_queries always`, custom SMTPat for refined-type quantifier). If still saturated, accept lax status as known Z3 limitation (document in MLKEM_STATUS.md). | 1–2 | **High**: Z3 saturation on 16-element forall is a historical blocker (attempted ≥3 times). Mitigation: consult SMT profiling + try different quantifier heuristics; if no progress in 60 min, defer to future Z3 upgrade. |
| **3** | AVX2 wrapper ops panic-freedom (4 fns at lines 335, 347, 367, 379): These are NOT in extraction scope (written in Rust spec fns, not hax-extracted). Verify that the wrapping logic (precondition chain → intrinsic call → postcondition discharge) is sound. Likely NOT needed for Milestone A (AVX2 dispatch already goes through trait posts). **Punt if no time.** | 0 (optional) | **Low**: Only if extra capacity. |
| **3–4** | Forward NTT layers 4-7 spec (USER-14 equivalent): Design spec for multi-step layer 4_plus (chunk-pair decomposition pattern, similar to inverse layer 2) + layer 7 (between-chunk butterfly). Likely 1 standalone spec design session + 1 lemma verification session. | 1–2 | **Medium**: Spec design is creative; implementation risk low (pattern mirrors inverse layers). |
| **4** | Integration + regression: Full `make` rebuild of `libcrux-ml-kem/proofs/`. Verify all `.checked` targets pass. Document final state in MLKEM_STATUS.md. Prepare Milestone B handoff (list of remaining Hacspec spec fns to design). | 0.5 | **Low**: Mechanical. |

### Top-3 Risk Items

1. **Z3 saturation on Pattern-1 eq_intro refactor** (Week 1): Even `Classical.forall_intro` form may hit incomplete-quantifier errors if the per-index lemma itself requires complex reasoning. **Mitigation**: Pre-test on `serialize_vector` in isolation; if no progress in 60 min, revert and file as USER-N for future Z3 improvement.

2. **Pattern-3 Phase C bridge blocks serialization fns** (Week 2): Hacspec-side bridge design is external. Source-side fixes can't land until bridge lands. **Mitigation**: Parallelize — work on Pattern-1 + Pattern-2 in weeks 1–2; Phase C bridge lands async; land source-side fix in week 3 if bridge ready, else defer 3 fns to week 4 buffer.

3. **Forward NTT layers 4-7 spec design** (Week 3–4): Multi-layer spec recursion requires careful definition (avoid spec non-termination, ensure Z3-decidable). Layer 7 is novel (between-chunk butterfly; no inverse analogue). **Mitigation**: Spec design sprint with cryptographer early in week 3; verify lemma in week 4.

### Success Metrics

- **By end of week 1**: 4 Pattern-1 fns panic-free, verified under `Classical.forall_intro` tactic.
- **By end of week 2**: 3 Pattern-2 fns panic-free; Pattern-3 fns awaiting Phase C bridge (tracked as blocker).
- **By end of week 3**: Forward NTT layer 4+ spec + lemmas verified; Portable NTT layer 1-2 status determined (panic_free or documented Z3 limitation).
- **By end of week 4**: Full Milestone A achieved. `make` clean rebuild. No new admits introduced (Pattern-1 + Pattern-2 proofs are real verification).

---

## Summary Table — Current State vs. Milestones

| Milestone | Below-Trait | Above-Trait | Spec | Est. Work |
|-----------|---|---|---|---|
| **Current (Phase E push 3)** | 28/30 panic_free (2 lax NTT layer ops); trait posts verified | 8/16 ind_cpa fns panic_free; 0 Hacspec ensures on 8 panic_free fns | 90 commute lemmas verified, 40 spec axioms | **0** |
| **Milestone A** | 30/30 panic_free | All 16 ind_cpa + 8 ind_cca dispatchers panic_free | (same as current) | **11–15 sessions** |
| **Milestone B** | (same as A) | Full Hacspec ensures on all 24 above-trait fns | Add 2 unpacked-API spec fns; cite all 24 fns | **+4–7 sessions** |

---

## Appendix: Files & Locations Referenced

- **Trait boundary**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/vector/traits.rs` (1195 lines)
- **Portable impl**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/vector/portable.rs` (1195 lines)
- **AVX2 impl**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/vector/avx2.rs` (1388 lines)
- **NEON impl**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/vector/neon.rs` (214 lines)
- **Ind_cpa API**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/ind_cpa.rs` (1200+ lines, 16 target fns)
- **Ind_cca API**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/src/ind_cca.rs` (950+ lines, mixed)
- **F* extraction Makefile**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/proofs/fstar/extraction/Makefile` (ADMIT_MODULES list, lines 5-21)
- **Hacspec specs**: `/Users/karthik/libcrux-trait-opacify/specs/ml-kem/proofs/fstar/extraction/` (16 modules, ~9344 total lines)
- **Phase D/E session reports**: `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/proofs/agent-status/session-2026-05-02-phaseD.md`, `session-2026-05-02-phaseE-cont.md`

---

**Report generated**: 2026-05-03 by read-only audit agent  
**Audit scope**: HEAD `b9eee5838` (Phase E push 3, 2026-05-03 03:09 UTC)

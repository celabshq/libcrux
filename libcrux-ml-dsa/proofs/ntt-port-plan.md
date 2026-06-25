# ML-KEM → ML-DSA NTT Proof Porting Plan

Generated 2026-04-30 by a planning agent reading `/Users/karthik/libcrux-trait-opacify`
(ML-KEM trait-opacify branch — most-advanced NTT proof state in this codebase).
Targets ML-DSA milestones 1, 2, 5, 6, 7 in `proof_milestones.md` (NTT/Invntt rows,
currently `📦 partial-spec`).

**Total estimate**: 30–45 agent-hours for full forward+inverse NTT closure once
the chunking lemma lands. Single-agent 3-sprint shape: forward (15 hr), inverse
(12 hr), composition + AVX2 mirror + cleanup (10 hr).

## 1. Inventory of ML-KEM's NTT/Invntt proof artifacts

### Hacspec specs (per-chunk + polynomial)
- `/Users/karthik/libcrux-trait-opacify/specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.Ntt.fst` (515 lines)
  - `butterfly` (FE-level Cooley-Tukey pair); `ntt_layer_n N p len zetas` (generic
    over N, parametrically used at N=16 within-chunk *and* N=256 polynomial);
    `ntt_layer p layer` (256-thin wrapper picking zeta slice); `ntt p` (chains
    layers 7..1).
- `/Users/karthik/libcrux-trait-opacify/specs/ml-kem/proofs/fstar/extraction/Hacspec_ml_kem.Invert_ntt.fst` (236 lines)
  - `inv_butterfly`; `ntt_inverse_layer_n`; `ntt_inverse_layer`;
    `ntt_inverse_butterflies`; `ntt_inverse`.

### Trait-level branch posts
- `/Users/karthik/libcrux-trait-opacify/libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Vector.Traits.Spec.fst`
  - `ntt_layer_{1,2,3}_step_branch_post`, `ntt_layer_{1,2,3}_step_post`
    (4 branches × 4 lanes per branch == 16 lanes), `ntt_layer_{1,2,3}_step_pre`;
    mirror set for `inv_ntt_layer_*_step_post`. Lines ~262–733.

### Per-lane / per-chunk commute lemmas in `Hacspec_ml_kem.Commute.Chunk.fst` (2668 lines)
Within-chunk lifters from "trait branch post" → "Hacspec ntt_layer_n at N=16":
- Helpers: `mont_array_lane`, `zetas_4_lane`, `lemma_ntt_layer_n_16_2_lane`,
  `lemma_ntt_layer_1_butterfly_to_fe`, `lemma_inv_ntt_layer_1_butterfly_to_fe`.
- Step bridges (existence-only, layer 2/3 in Bridges.fst):
  `lemma_ntt_layer_{1,2,3}_step_chunk_commutes`,
  `lemma_inv_ntt_layer_{1,2,3}_step_chunk_commutes`.
- Layer 1 forward bridge end-to-end: `lemma_ntt_layer_1_step_lane_bridge` +
  `lemma_ntt_layer_1_step_to_hacspec` (lines 2292, 2342).

### Per-vector function-form bridges in `Hacspec_ml_kem.Commute.Bridges.fst` (1208 lines)
- Inverse: `lemma_inv_ntt_layer_{1,2,3}_step_to_hacspec` (lines 252, 702, 411).
- Forward: `lemma_ntt_layer_{1,2,3}_step_to_hacspec` (lines 150, 984, 1129).
- Cross-chunk lane-bridge: `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec`
  (line 1173) — used by `invert_ntt_at_layer_4_plus`.

### Wrapper `ensures` citations
- `Libcrux_ml_kem.Ntt.fst`: `ntt_at_layer_{1,2,3}_` cite
  `lemma_ntt_layer_{1,2,3}_step_to_hacspec` (lines 119, 231, 334).
  Layer 4_plus + 7 are bounds-only.
- `Libcrux_ml_kem.Invert_ntt.fst`: `invert_ntt_at_layer_{1,2,3}` proven;
  layer 4_plus is `--admit_smt_queries true` (USER-14); top-level
  `invert_ntt_montgomery` is `--admit_smt_queries true` (USER-15).
- **No `lemma_ntt_full_commute` / `lemma_inv_ntt_full_commute` polynomial-level
  composition lemma is proven in ML-KEM** (this is exactly USER-15).

### Sprint docs
- `manual-proof-targets.md`, `commutativity-handoff.md`,
  `simd-model-unification-plan.md`, `proof_milestones.md`,
  `agent-status/handoff-2026-04-2{8,9}*.md`.
- Z3 divergence at "split queries 5+ stalling past 15 min" is the dominant
  pattern.

## 2. Diff vs ML-DSA — structural mismatches

| Property | ML-KEM | ML-DSA |
|---|---|---|
| Modulus q | 3329 (i16, fits u16) | 8380417 (i32) |
| Layers | 7 (`layer 7..1` in `ntt`) | 8 (`layer 7..0` in `ntt`) |
| Lane width per SIMD unit | 16 i16 | 8 i32 |
| Chunks per polynomial | 16 chunks × 16 lanes | 32 chunks × 8 lanes |
| `bit_rev` | `bit_rev_7_` | `bit_rev_8_` |
| Within-chunk layers | 1, 2, 3 (len=2,4,8 < 16) | 0, 1, 2 (len=1,2,4 < 8) |
| Cross-chunk layers | 4_plus, 5, 6, 7 | 3, 4, 5, 6, 7 |
| Trait NTT entry-point | per-layer `f_ntt_layer_{1,2,3}_step` (within-chunk) + free fns above the trait (cross-chunk) | **monolithic `Operations::ntt(simd_units)`** — full 8-layer NTT in one call (no per-layer trait method) |
| Per-layer impl helpers | `ntt_at_layer_{1..7}` in `src/ntt.rs` (above trait) | `ntt_at_layer_{0..7}` in `src/simd/portable/ntt.rs` (**inside the trait impl**, opaque-to-SMT) |
| Hacspec at-layer signature | `ntt_layer_n N p len zetas` (slice of zetas) | `ntt_layer p layer` (single `layer` arg, indexes `v_ZETAS` of length 256 directly) |
| FE form for spec values | `mont_i16_to_spec_fe x = (x · 169) mod 3329` (Montgomery → plain) | impl coefficients are *already* in standard domain at NTT entry; mod_q wraps; `Hacspec_ml_dsa.Arithmetic.mod_q` |
| Hacspec spec is in flat-array form | No — at-layer spec is per-chunk-or-poly | **Yes** — `Hacspec_ml_dsa.Ntt.ntt_layer p layer` operates on `t_Array i32 256` directly |

**Key consequence — the porting model is *not* a one-to-one rename:**
- ML-KEM's per-chunk Hacspec `ntt_layer_n (mk_usize 16) ...` does not have a
  direct ML-DSA analog. ML-DSA's at-layer Hacspec is **already** the
  polynomial-level `ntt_layer p layer`. The ML-DSA chunking lemma must therefore
  relate "32 SIMD units of 8 lanes each" ↔ "flat 256 array" *upstream* of the
  per-layer call site, not within each per-chunk step.
- ML-DSA has no per-layer trait method to attach per-layer step bridges to.
  The trait method `Operations::ntt` is the **whole NTT**. Per-layer bridges
  instead attach to the **above-trait helper functions**
  `Libcrux_ml_dsa.Simd.Portable.Ntt.ntt_at_layer_{0..7}_` (and the AVX2 mirror),
  each of which is opaque-to-smt and currently bounds-only. These per-layer
  functions correspond 1:1 to `Hacspec_ml_dsa.Ntt.ntt_layer p layer`.

## 3. Reusability matrix

| ML-KEM artifact | ML-DSA target | Class | Notes |
|---|---|---|---|
| `mont_i16_to_spec_fe`, `mont_i16_to_spec_array` | n/a (impl already in standard domain at NTT entry) | **N/A** | Drop the Montgomery lift entirely for forward NTT. Inverse NTT does need a `to_standard_domain` style lift at the *exit*, but only a single-poly final lemma. |
| `lemma_ntt_layer_n_16_2_lane` (lane unfold of `ntt_layer_n` at N=16, len=2) | n/a | **N/A** | ML-DSA Hacspec has no per-chunk `ntt_layer_n N` family; the 8-lane within-chunk steps unfold directly off `Hacspec_ml_dsa.Ntt.ntt_layer p 0` (and `p 1`, `p 2`) at flat-array indices. |
| Trait `ntt_layer_{1,2,3}_step_branch_post` + `_post` | new per-SIMD-unit branch posts for layers 0/1/2 (or stronger posts on existing `simd_unit_ntt_at_layer_{0,1,2}`) | **TEMPLATE-PORT** | Same shape (per-branch FE eqs collapsed by `forall_intro`). Different lane count (8 vs 16) and FE form (plain mod_q, no Mont lift). ~3-4 hr per layer. |
| `lemma_ntt_layer_1_step_to_hacspec` per-vector bridge | per-SIMD-unit bridge: `Coefficients_ntt_at_layer_0_step_to_hacspec` for ML-DSA layer 0 (and similarly layers 1, 2) | **TEMPLATE-PORT** | Reuse the `lane_bridge → forall_intro → Seq.lemma_eq_intro` skeleton. Different: target unfolds `Hacspec_ml_dsa.Ntt.ntt_layer p 0` at the 8 lanes of *one chunk*, while bookkeeping that the other 31 chunks are unchanged at this layer. ~3 hr per (layer, direction) pair. |
| `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec` (cross-chunk pair bridge for layer 4_plus inverse) | inverse layer 3..7 cross-chunk pair bridge | **TEMPLATE-PORT** | ML-DSA's cross-chunk butterflies (layers 3..7) are the analog. Same `inv_butterfly._{1,2}` per-lane unfold pattern. ~2 hr. |
| `lemma_intt_mont_finalize_fe`, `lemma_subtract_reduce_*` (Montgomery exit handling for inverse NTT) | similar inverse-NTT exit lemma using `Hacspec_ml_dsa.Ntt.reduce_polynomial` | **REWRITE** (not direct port; semantically `× 256⁻¹ mod q` rather than `× R⁻¹ mod q`) | Constants differ (8347681 vs 1441). Structure preserved. ~3-4 hr. |
| `lemma_add_chunk_commutes_*`, `lemma_sub_chunk_commutes_*`, `lemma_montgomery_multiply_by_constant_chunk_commutes_*`, `lemma_butterfly_pair_commute`, `lemma_inv_butterfly_pair_commute` | already done in `Hacspec_ml_dsa.Commute.Chunk.fst`: `lemma_{add,sub}_lane_commute`, `lemma_mont_mul_bound_and_mod_q` | **N/A (already in place)** | The per-lane primitive layer is the most "already-done" piece on ML-DSA. |
| `Hacspec_ml_kem.Commute.Bridges.fst` file structure (separate file to isolate Z3 divergence; depends on Chunk) | Create `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Bridges.fst` | **DIRECT-PORT (file scaffold only)** | New module sourced from existing `Hacspec_ml_dsa.Commute.Chunk.fst` and `Libcrux_ml_dsa.Simd.Traits.Specs`. ~1 hr. |
| Wrapper `ntt_at_layer_*` ensures citations in `Libcrux_ml_kem.Ntt.fst` (that cite the bridge) | wrapper `Libcrux_ml_dsa.Simd.Portable.Ntt.ntt_at_layer_*` ensures (and the Avx2 mirror) | **TEMPLATE-PORT** | Same `aux+forall_intro` discharge pattern. The hashing change: ML-KEM stays opaque per-chunk (uses `f_repr` indexed by `Seq.index`); ML-DSA uses `f_repr` of `Coefficients` (the 8-lane vector type). Per-step ~1.5 hr once the bridge lands. |
| `lemma_ntt_full_commute` (NOT in ML-KEM yet) | required for ML-DSA milestone 1 | **NEW WORK** (no source) | This must be authored fresh. ML-KEM's USER-15 is the closest precedent — but it too is unfinished. Must compose `Hacspec_ml_dsa.Ntt.ntt = ntt_layer ... 7 ... 0` with 8 per-layer bridges by simple let-binding chain in F*. ~3-5 hr once layers exist. |
| Compression / decompression chunk_commute lemmas | n/a (ML-DSA has no compress/decompress; uses bit-pack encoders instead) | **N/A** | |
| ML-KEM Polynomial-level `lemma_poly_barrett_reduce_commute` | already exists at lane-level in `Hacspec_ml_dsa.Commute.Chunk.fst` line 23 (`lemma_reduce_lane_commute`); needs **TEMPLATE-PORT** of the chunk→poly lift | **TEMPLATE-PORT** | Same `forall_intro` + `Seq.lemma_eq_intro` lift. |
| `simd-model-unification-plan.md` migration (Model A → Model B) | n/a — ML-DSA *is* Model B; nothing to do | **N/A (favourable)** | Directly relevant: ML-DSA proofs do not have to wait for the unification. |

## 4. Concrete porting steps (ordered)

| # | Step | Class | Hours |
|---|---|---|---|
| 1 | Author chunking lemma `simd_units_to_array` (32 × `Coefficients` ↔ flat `t_Array i32 256`) + inverse `array_to_simd_units` + reveal lemmas (`Seq.index a (8b+l) == Seq.index ((Seq.index simd_units b).f_values) l`). Place in `Hacspec_ml_dsa.Commute.Chunk.fst` after the existing per-lane lemmas (~line 660). | NEW | 2–3 |
| 2 | Create new module `Hacspec_ml_dsa.Commute.Bridges.fst` with the same architectural rationale as ML-KEM's: isolate the slow Z3 queries from per-lane lemmas downstream consumers depend on. | DIRECT-PORT (scaffold) | 0.5 |
| 3 | Author per-SIMD-unit lane bridge for **forward layer 0** (within-chunk, len=1). 8 lanes of FE eqs ↔ `(Hacspec_ml_dsa.Ntt.ntt_layer flat 0)` indices `8b..8b+7`. Mirror `lemma_ntt_layer_1_step_lane_bridge` shape; FE form is plain `mod_q`. Strengthen `simd_unit_ntt_at_layer_0` ensures (`Libcrux_ml_dsa.Simd.Portable.Ntt`). | TEMPLATE-PORT | 3–4 |
| 4 | Compose 32 per-SIMD-unit lane bridges → polynomial-level layer-0 bridge `lemma_ntt_layer_0_step_to_hacspec_poly`. Pattern: `Classical.forall_intro aux + Seq.lemma_eq_intro` over 256 lanes (cf. ML-KEM `lemma_ntt_layer_1_step_to_hacspec`). | TEMPLATE-PORT | 1.5–2 |
| 5 | Strengthen `Libcrux_ml_dsa.Simd.Portable.Ntt.ntt_at_layer_0_` ensures from bounds-only to `simd_units_to_array re_future == Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array re) 0`. Body discharge via lemma from step 4. | TEMPLATE-PORT | 1.5 |
| 6 | Repeat steps 3–5 for forward **layer 1** (within-chunk, len=2). Slightly different per-branch unfold pattern. | TEMPLATE-PORT | 3–4 |
| 7 | Repeat for forward **layer 2** (within-chunk, len=4). | TEMPLATE-PORT | 3–4 |
| 8 | Author cross-chunk pair bridge for **layer 3** (len=8 ≥ chunk-width 8 — first cross-chunk layer; one butterfly per lane *across two chunks*). Mirror `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec`. Wire into `ntt_at_layer_3_` ensures. | TEMPLATE-PORT | 2.5–3 |
| 9 | Repeat step 8 for layers **4, 5, 6, 7** (cross-chunk). All structurally identical (same pair bridge, different `STEP`/`STEP_BY` constants and zeta indices). Should template easily. | TEMPLATE-PORT | 1.5 hr × 4 = 6 |
| 10 | Author `lemma_ntt_full_commute`: chains 8 layer ensures into top-level `ntt`/`SIMDUnit::ntt` (Portable backend). Definitional unfold of `Hacspec_ml_dsa.Ntt.ntt = ntt_layer ... 7 ... 0`. | NEW (no ML-KEM analog complete) | 3–5 |
| 11 | Strengthen `src/ntt.rs::ntt` `#[hax_lib::ensures]` to cite `Hacspec_ml_dsa.Ntt.ntt`. Strengthen the SIMD trait `Operations::ntt` post (`src/simd/traits.rs:331`). Update Avx2 and Portable trait impl posts. | TEMPLATE-PORT | 2–3 |
| 12 | Mirror entire pipeline for inverse NTT (steps 3–10): 8 per-layer Gentleman–Sande bridges + `lemma_intt_full_commute`. Inverse direction is structurally simpler within-chunk (no zeta-pre-mul) but adds the `reduce_polynomial` (× 256⁻¹) finalize step. | TEMPLATE-PORT | ~70% of forward effort = 18–25 |
| 13 | Wire `invert_ntt_montgomery` ensures to cite `Hacspec_ml_dsa.Ntt.intt`. | TEMPLATE-PORT | 2 |
| 14 | (Optional / next-sprint) Mirror the AVX2 backend per-layer bridges. Most lemmas are reusable verbatim because both backends share `Coefficients` shape; the difference is `core-models` SMTPats firing. | TEMPLATE-PORT | 4–8 |

**Critical-path lemma**: **Step 1 (chunking lemma `simd_units_to_array`)**.
Without it, no per-layer bridge can be stated, let alone proven. ML-KEM never
needed this because ML-KEM's per-layer Hacspec is itself per-chunk; ML-DSA's
per-layer Hacspec is poly-flat, so the chunk↔flat translation is mandatory and
load-bearing.

## 5. Risk flags

1. **Trait granularity mismatch (highest risk)**. ML-KEM has trait methods at
   the per-layer-per-chunk level (`f_ntt_layer_1_step`); ML-DSA has trait
   methods only at the whole-NTT level (`Operations::ntt`). Per-layer bridges
   in ML-DSA must therefore live at the **above-trait helper** level
   (`Libcrux_ml_dsa.Simd.Portable.Ntt.ntt_at_layer_*`), not at the trait. This
   means each backend (Portable, AVX2) needs its own per-layer bridges; the
   bridges are not generic over the trait. Counts as 2× the work multiplier vs
   ML-KEM's trait-generic bridges.

2. **`opaque_to_smt` wall**. All `simd_unit_ntt_at_layer_*` and `ntt_at_layer_*`
   are tagged `[@@ "opaque_to_smt"]`. Strengthening their ensures requires
   `reveal_opaque` plumbing at every call site or strategic non-opaque proxy
   lemmas. Mirror the ML-KEM pattern: prove a non-opaque `*_post` predicate,
   then use `reveal_opaque` in the layer-7 entry chain.

3. **Z3 divergence at within-chunk lane bridges**. ML-KEM's layer 2/3 forward
   bridges famously stalled past 2.7 minutes per query without progress (cited
   in ML-KEM Chunk.fst lines 2382-2403). ML-DSA's i32 modulus 8380417 does
   **not** fit in the same SMT-friendly range as i16/3329, so non-linear
   arithmetic across `mod_q` will be slower. Plan for `--split_queries always`
   + `--ext context_pruning` from day one and budget rlimit ≥ 800.

4. **"trait-opacify branch is unmerged sprint state"**. ML-KEM's most-advanced
   NTT proof state is on a *worktree* (`/Users/karthik/libcrux-trait-opacify`),
   not on main. The branch is mid-sprint with multi-agent activity; recipes
   are still being refined (handoff docs reference unresolved USER-13/14/15).
   Treat their patterns as battle-tested but not finalised — be ready to
   follow-up rather than treat them as canonical.

5. **simd-model-unification-plan.md is **deferred** ML-KEM-side work** (status
   line 3: "DEFERRED. Pick this up after the trait layer is stabilized"). It
   does **not** affect ML-DSA — ML-DSA is already on Model B. But if you mirror
   an ML-KEM lemma that relies on Model A's `vec256_as_i16x16` axiom, swap to
   `to_i32x8` from `core-models` directly. The `Spec.Intrinsics.fsti` SMTPat
   library (62 lemmas, ML-DSA-side) auto-fires at lane level — leverage it
   instead of porting Model A workarounds.

6. **Sliced API constraint** (per `feedback_no_matrix_array_refactor`). Do not
   refactor `&[Coefficients]` slice signatures to `[Coefficients; 32]` arrays.
   The chunking lemma must operate on slice / `t_Array Coefficients (mk_usize 32)`
   indexed via `Seq.index`, not via array projection.

7. **Inverse NTT Montgomery exit**. ML-KEM's `lemma_subtract_reduce_commute`
   uses `RR_div_128 = 1441 mod 3329`; ML-DSA's reduce_polynomial uses
   `8347681 mod 8380417`. Direct constant replacement only — but the proof of
   the constant identity (`8347681 ≡ 256⁻¹ mod 8380417`) must be authored:
   simple `assert_norm`, ~10 min.

## 6. Recommended sprint shape

**Layer-by-layer, forward-direction first, within-chunk first.**

Reasoning:
- The chunking lemma is the single largest risk; landing it on **layer 0
  forward** gives the simplest possible test (len=1, no zeta complexity, 8
  independent butterflies per chunk). If the chunking lemma is wrong, layer 0
  will catch it in a few hours rather than discovering it at layer 7 after days.
- Within-chunk first because the per-chunk bridge is the harder design
  (involves the chunking-lemma-for-one-chunk-only + 8 lanes); cross-chunk is
  structurally simpler (mirror of ML-KEM's
  `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec` which is just `()` once
  the per-lane FE eqs are produced).
- Forward first because forward NTT lives in standard domain at entry (no
  Mont-form gymnastics) — strictly easier than inverse, which requires the
  `× 256⁻¹` finalize.
- Layer-by-layer (vs big-bang) because each layer's bridge is independently
  provable and citable; bisect-friendly. ML-KEM's experience shows cross-layer
  dependencies are minimal once the per-layer bridge skeleton lands.

After layer 0 forward demonstrates the pipeline, layers 1, 2 follow in
~6–8 hr. Cross-chunk layers 3–7 should batch easily (~6 hr total). Then
forward-NTT full-commute composition (~3–5 hr). Inverse follows the same shape.

**Single-agent 3 sprint estimate**: forward NTT in 1 sprint (~15 hr), inverse
NTT in 1 sprint (~12 hr), composition + AVX2 mirror + cleanup in 1 sprint
(~10 hr). **Total: ~37 agent-hours** to close milestones 1, 2, 5, 6, 7.
Multi-agent parallelism can compress to 2 sprints once the chunking lemma is
shared.

---

## Recommended FIRST commit (next session, ≤ 3 hours)

**Goal**: Land the chunking lemma + a single-direction reveal lemma. This is
the critical-path artifact and has no SMT-divergence risk; it's pure structural
definitions.

**Deliverable**: a single commit on a fresh branch (e.g.,
`agent/ntt-chunking-lemma`) modifying ONLY
`/Users/karthik/libcrux-ml-dsa-proofs/specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
to add:

1. `let simd_units_to_array (simd_units: t_Array Coefficients (mk_usize 32))
   : t_Array i32 (mk_usize 256)` — `createi 256` indexing
   `Seq.index (Seq.index simd_units (i / 8)).f_values (i % 8)`.
2. `lemma_simd_units_to_array_reveal (simd_units) (b: nat{b < 32}) (l: nat{l < 8})`
   proving `Seq.index (simd_units_to_array simd_units) (8*b + l) ==
   Seq.index (Seq.index simd_units b).f_values l`. Single-line `createi_lemma`
   invocation.
3. `lemma_simd_units_to_array_other_chunk_unchanged` — given `forall j ≠ b.
   simd_units_future[j] == simd_units[j]` and
   `simd_units_future[b].f_values == updated`, derive `forall i. (i / 8 ≠ b)
   ==> (simd_units_to_array simd_units_future)[i] ==
   (simd_units_to_array simd_units)[i]`. This is what every per-layer bridge
   will cite to bookkeep "the other 31 chunks didn't change".
4. (Optional, if time permits within 3 hr) inverse direction
   `array_to_simd_units` + a round-trip identity lemma
   `simd_units_to_array (array_to_simd_units a) == a`.

**Acceptance criterion**: Module typechecks with no `admit ()` and runs in
`make check` with rlimit ≤ 80 (these are pure structural lemmas; should not
need higher). Nothing in the impl tree changes.

**Why this is safe and high-leverage**: It cannot break any existing proof
(purely additive in a leaf module's tail), it has no Z3-divergence risk (no
non-linear arithmetic), and every subsequent per-layer bridge will cite at
least lemmas (2) and (3). Once committed, parallel agents can independently
start authoring the layer-0/1/2 forward bridges (steps 3–7 of the plan above)
without further coordination.

---

### Critical Files for Implementation
- `/Users/karthik/libcrux-ml-dsa-proofs/specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
  — chunking lemma + per-layer step bridges go here (or in a new `Bridges.fst`
  peer)
- `/Users/karthik/libcrux-ml-dsa-proofs/specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.Ntt.fst`
  — read-only spec; the target of every bridge's `ensures`
- `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/proofs/fstar/extraction/Libcrux_ml_dsa.Simd.Portable.Ntt.fst`
  — owner module for `ntt_at_layer_{0..7}_` ensures strengthening (Portable
  backend)
- `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/src/simd/portable/ntt.rs`
  — Rust source where `#[hax_lib::ensures]` annotations are written for the
  per-layer functions; corresponding AVX2 file
  `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/src/simd/avx2/ntt.rs`
- `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/src/simd/traits.rs`
  — final ensures strengthening of the trait `ntt`/`invert_ntt_montgomery`
  posts (lines 331, 342) is the closing milestone

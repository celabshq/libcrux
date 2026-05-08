# Audit — ml-kem trait abstraction boundary (`vector/traits.rs`)

**Date:** 2026-05-08
**Tip:** `1d118cf8e` (deserialize_5 outer fully verified)
**Audit basis:** the user's explicit abstraction policy:

> Modules above `traits.rs` should never need to reason about the concrete
> bounds of i16 values. They should only use `is_i16b_array_opaque` and
> friends. Similarly they should not see raw arithmetic operations like
> `+`, `%`, etc. and instead should have to work with opaque predicates
> and lemmas on them. These rules can easily be broken by careless SMTPats.

This audit covers four scope items:
1. Annotations on `traits.rs` that expose raw arithmetic / per-i16 bounds
   above the trait.
2. Lemmas about polynomials (`polynomial.rs`) that violate or weaken the
   boundary.
3. Every SMTPat visible above `traits.rs` whose pattern can fire on
   above-trait shapes.
4. Functions with universal quantifiers in pre/post that could be
   replaced with opaque atoms.

## Summary

| Status | Item count |
|---|---|
| **Already opaque (good)** | 21 named predicates / lemmas |
| **SMTPat — disciplined dual-pattern (acceptable)** | 5 lemmas (1 in `traits.rs`, 4 in `polynomial.rs`) |
| **LEAKY pre/post — raw `v` + arithmetic above trait** | **10 functions in `traits.rs::spec`** |
| **Universal-quantifier refactor candidates** | 6 (overlap with leaky list) |
| **Mild leak — transparent intermediate predicates** | 1 (`is_bounded_poly`) |

Headline finding: **the per-lane FE/branch posts and the bound predicates
are well opacified, but 10 of the 16-lane "primitive" trait
methods (`add`, `sub`, `negate`, `multiply_by_constant`,
`cond_subtract_3329`, `barrett_reduce`, `montgomery_multiply_by_constant`,
`to_unsigned_representative`, `compress_1`, plus the per-element bound
forms) state their pre/post via raw `v (Seq.index ...) + v (Seq.index ...)`
patterns** and so leak `v`, `+`, `-`, `*`, `==`, `>=`, `<=` to every
caller above the trait. This is the highest-priority cleanup target.

## 1. `traits.rs` annotations — leaky pre/post

### 1.1 LEAKS — raw `v` + arithmetic in pre/post

For each, the leak is the **pre/post body** as seen above the trait. Callers
above `traits.rs` that invoke these methods receive the raw equation.

| # | Function (`vector/traits.rs::spec`) | Lines | Leak |
|---|---|---|---|
| L1 | `add_pre` | 617-623 | `forall i. is_intb (pow2 15 - 1) (v (Seq.index lhs i) + v (Seq.index rhs i))` — raw `v`, `+` |
| L2 | `add_post` | 633-640 | `forall i. v (result i) == v (lhs i) + v (rhs i) /\ is_intb …` — raw `v`, `+`, `==` |
| L3 | `sub_pre` / `sub_post` | 644-659 | same as L1/L2 with `-` |
| L4 | `negate_pre` / `negate_post` | 661-675 | raw `v`, unary `-`, `==` |
| L5 | `multiply_by_constant_pre` / `_post` | 677-695 | raw `v`, `*`, `==` |
| L6 | `cond_subtract_3329_post` | 724-738 | per-i `v y == v x - 3329 \/ v y == v x` — raw `-` and value comparison |
| L7 | `barrett_reduce_post` | 744-751 | `mod_q_eq (v r[i]) (v v[i])` — opaque mod-q (better) but still exposes `v` |
| L8 | `montgomery_multiply_by_constant_post` | 763-774 | `mod_q_eq (v r[i]) (v vec[i] * v c * 169)` — raw `v`, `*` (`* 169` is the magic Montgomery factor) |
| L9 | `to_unsigned_representative_post` | 789-799 | per-i `v y >= 0 /\ v y <= 3328 /\ mod_q_eq (v y) (v x)` — raw `v`, `>=`, `<=` |
| L10 | `compress_1_pre` | 801-807 | `forall i. v (vec i) >= 0 /\ v (vec i) < 3329` — raw `v`, `>=`, `<`, plus a magic constant |

### 1.2 ALREADY OPAQUE (good — these set the bar)

The cleanup targets in §1.1 should look like these:

| Predicate | Lines | Pattern |
|---|---|---|
| `is_i16b_array_opaque` | 147-149 | `[@@ "opaque_to_smt"]` over `forall i. is_i16b l (Seq.index x i)` |
| `bounded_i16_array` | 166-169 | `[@@ "opaque_to_smt"]` over `forall i. v lo <= v (x i) /\ v (x i) <= v hi` |
| `bounded_abs_i16_array` / `bounded_pos_i16_array` | 192-196 | thin wrappers over `bounded_i16_array` (still opaque) |
| `compress_post_N` / `decompress_post_N` | 214-233 | `[@@ "opaque_to_smt"]` over per-i `i16_to_spec_fe` equation |
| `compress_1_lane_post` / `compress_d_lane_post` | 310-320 | per-lane FE equation, opaque |
| `decompress_1_lane_post` / `decompress_d_lane_post` | 322-334 | same shape |
| `ntt_layer_{1,2,3}_step_branch_post` | 343-453 | per-branch FE-butterfly, opaque |
| `inv_ntt_layer_{1,2,3}_step_branch_post` | 455-553 | same shape |
| `ntt_multiply_branch_post` | 555-606 | per-branch ntt-multiply, opaque |
| `cond_subtract_3329_pre` | 720-722 | uses `is_i16b_array_opaque (pow2 12 - 1)` |
| `barrett_reduce_pre` | 740-742 | uses `is_i16b_array_opaque 28296` |
| `to_unsigned_representative_pre` | 776-777 | uses `is_i16b_array_opaque 3328` |
| `montgomery_multiply_by_constant_pre` | 759-761 | uses `is_i16b 1664 c` (single value, not per-element) |
| `compress_pre`, `compress_post`, `decompress_*_post`, `decompress_*_pre` | 817-895 | use `bounded_i16_array` / `bounded_pos_i16_array` + opaque `*_lane_post` + transparent `forall16` |
| `ntt_layer_*_pre`/`_post`, `inv_ntt_layer_*_pre`/`_post`, `ntt_multiply_pre`/`_post` | 897-1086 | use `is_i16b_array_opaque (k*3328)` + opaque `*_branch_post` + transparent `forall4` |
| `serialize_*` / `deserialize_*` (1, 4, 10, 12) | 1088-1198 | delegate to `serialize_pre_N` / `serialize_post_N` / `deserialize_post_N` (opaque inner predicates) |

The contrast is clear: the **n=16-vector linear-arithmetic primitives**
(`add`, `sub`, `negate`, `multiply_by_constant`) are stated raw, while the
**polynomial-level / hacspec-level** methods are stated through opaque
atoms.

### 1.3 SMTPat in `traits.rs` (1 lemma)

| Lemma | Lines | Trigger | Verdict |
|---|---|---|---|
| `lemma_bounded_i16_array_lookup` | 176-180 | `[SMTPat (Seq.index x i); SMTPat (bounded_i16_array lo hi x)]` | **Disciplined dual-pattern.** Fires only when BOTH `Seq.index x i` AND `bounded_i16_array lo hi x` are in the e-graph. Yield: `v lo <= v (Seq.index x i) /\ v (x i) <= v hi`. **This yield is itself a leak** — callers above the trait now have raw `v` and `<=`. Acceptable because the use-case is consume-only (read a per-lane bound from an existing opaque atom), but the yield should ideally be wrapped in a per-lane opaque atom (e.g. `i16_in_range lo hi (Seq.index x i)`). |

The `lemma_bounded_i16_array_intro` companion (lines 186-190) has **no SMTPat** — explicit at call sites. Good discipline.

## 2. `polynomial.rs` SMTPats and lemmas

### 2.1 SMTPat lemmas (4)

| Lemma | Lines | Trigger | Yield | Verdict |
|---|---|---|---|---|
| `lemma_is_bounded_polynomial_vector_elim` | 160-176 | `[SMTPat (Seq.index arr (v i)); SMTPat (is_bounded_polynomial_vector v_RANK #v_Vector b arr)]` | `is_bounded_poly b (Seq.index arr (v i))` | **Disciplined dual-pattern.** Yield is *opaque atom* `is_bounded_poly` (still transparent forall — see §2.3). |
| `lemma_is_bounded_polynomial_matrix_elim` | 178-194 | `[SMTPat (Seq.index m (v i)); SMTPat (is_bounded_polynomial_matrix v_RANK #v_Vector b m)]` | `is_bounded_polynomial_vector v_RANK #v_Vector b (Seq.index m (v i))` | Disciplined; yield is opaque. |
| `lemma_is_bounded_polynomial_vector_elim_nat` | 199-218 | `[SMTPat (Seq.index arr i); SMTPat (is_bounded_polynomial_vector v_RANK #v_Vector b arr)]` | `is_bounded_poly b (Seq.index arr i)` | Disciplined; nat-indexed convenience. |
| `lemma_is_bounded_polynomial_matrix_elim_nat` | 220-237 | `[SMTPat (Seq.index m i); SMTPat (is_bounded_polynomial_matrix v_RANK #v_Vector b m)]` | `is_bounded_polynomial_vector v_RANK #v_Vector b (Seq.index m i)` | Disciplined; nat-indexed. |

These are correctly designed: **dual triggers, both must match**, and the
**yield is itself an opaque-atom predicate** (no raw arithmetic, no per-i16
bound exposed). Good discipline. ✅

### 2.2 Intro / higher lemmas (no SMTPat — explicit calls)

`lemma_is_bounded_polynomial_{vector,matrix}_intro` (lines 82-114),
`is_bounded_vector_higher` / `is_bounded_poly_higher` (lines 118-134),
`lemma_decompress_post_to_is_bounded_vector` (lines 271-298),
`lemma_is_bounded_polynomial_vector_higher` / `lemma_is_bounded_polynomial_matrix_higher` (lines 245-308). All gated by explicit call sites. ✅

### 2.3 Mild leak — `is_bounded_poly` is transparent

`is_bounded_poly` (lines 45-53) is defined as:

```rust
forall (i:nat). i < 16 ==> is_bounded_vector b (p.f_coefficients.[sz i])
```

This is **not** marked `[@@ "opaque_to_smt"]` — it's a transparent
forall over the opaque `is_bounded_vector` atom.

**Effect on callers above the trait:** when the SMTPat'd elim
(§2.1) yields `is_bounded_poly b (Seq.index arr i)`, Z3 immediately
unfolds it to `forall j. j < 16 ==> is_bounded_vector b (p.f_coefficients[j])`.
The forall body is opaque (`is_bounded_vector` cites
`is_i16b_array_opaque`), but the forall itself is now in scope, and Z3
will instantiate it on demand for any `Seq.index p.f_coefficients j`
the caller mentions.

**Verdict:** mild leak. The consumer pays a per-`f_coefficients[j]`
quantifier instantiation cost. Better than raw arithmetic but worse than
a fully opaque `is_bounded_poly_opaque b p` atom.

**Recommended fix:** mark `is_bounded_poly` opaque, expose elim/intro
lemmas with dual-trigger SMTPats analogous to §2.1. Migration cost: every
caller that currently gets `is_bounded_vector` via auto-unfold needs
explicit reveal or explicit lemma call. Estimate ~10–20 sites across the
codebase.

## 3. SMTPats above `traits.rs` — comprehensive list

Five SMTPat'd lemmas reach modules above the trait:

1. `Vector.Traits.Spec::lemma_bounded_i16_array_lookup` (§1.3)
2. `polynomial.rs::lemma_is_bounded_polynomial_vector_elim` (§2.1)
3. `polynomial.rs::lemma_is_bounded_polynomial_matrix_elim` (§2.1)
4. `polynomial.rs::lemma_is_bounded_polynomial_vector_elim_nat` (§2.1)
5. `polynomial.rs::lemma_is_bounded_polynomial_matrix_elim_nat` (§2.1)

All five use **dual-pattern triggers** (the disciplined form). None has a
single-pattern SMTPat over an opaque-atom predicate (the documented
anti-pattern in `feedback_smtpat_percent_above_trait`).

The only concern is #1's yield exposing raw `v lo <= v (x i)` — see §1.3.

## 4. Universal-quantifier refactor candidates

Candidates where a `forall i.` over per-i16 bounds/equations could be
replaced with an opaque atom:

| Function | Current shape | Proposed opaque atom |
|---|---|---|
| `add_pre` (L1) | `forall i. is_intb (pow2 15 - 1) (v lhs[i] + v rhs[i])` | `is_i16b_array_sum_safe lhs rhs (pow2 15 - 1)` |
| `add_post` (L2) | `forall i. v result[i] == v lhs[i] + v rhs[i] /\ is_intb …` | `i16_array_eq_sum result lhs rhs` (opaque), or split: an opaque equation atom + the existing `is_i16b_array_opaque` for the bound |
| `sub_pre` / `sub_post` (L3) | analogous to L1/L2 | `is_i16b_array_diff_safe`, `i16_array_eq_diff` |
| `negate_pre` / `_post` (L4) | `forall i. is_intb _ (v vec[i])` and `v result[i] == - v vec[i]` | `is_i16b_array_opaque _ vec` already exists for the pre; for the post: `i16_array_eq_neg result vec` opaque |
| `multiply_by_constant_pre` / `_post` (L5) | `forall i. is_intb _ (v vec[i] * v c)` and `v result[i] == v vec[i] * v c` | `is_i16b_array_mul_safe vec c`, `i16_array_eq_mul result vec c` |
| `compress_1_pre` (L10) | `forall i. v vec[i] >= 0 /\ v vec[i] < 3329` | Already half-done — should reuse `bounded_i16_array (mk_i16 0) (mk_i16 3328) vec`. The current form is a 1-off raw inline. |
| `cond_subtract_3329_post` (L6) | `forall i. v y == v x - 3329 \/ v y == v x` | `cond_subtract_3329_lane_post x y` opaque (mirrors `compress_1_lane_post` shape) |
| `to_unsigned_representative_post` (L9) | `forall i. v y >= 0 /\ v y <= 3328 /\ mod_q_eq …` | `to_unsigned_lane_post x y` opaque + the existing opaque mod_q_eq |

Refactor approach (per item):
1. Define an opaque atom (lane-form OR array-form, mirroring the
   `compress_1_lane_post` / `bounded_i16_array` pattern).
2. Provide a dual-pattern SMTPat'd elim lemma that yields concrete bounds
   when needed.
3. Rewrite the trait pre/post to cite the opaque atom.
4. Update every above-trait caller that previously consumed the raw
   `forall i. v (...)` form. Most callers should already be using
   `is_i16b_array_opaque` / `bounded_i16_array` for the bound part, so
   the equation-side rewrites are the main cost.

## Recommended cleanup order (prioritised)

| Priority | Item | Rationale |
|---|---|---|
| 1 | **L1–L5** (add/sub/negate/multiply_by_constant pre/post) | Most invoked path; touched by every polynomial-level chain (NTT, Barrett, Montgomery). Single recipe applies to all 5. |
| 2 | **L10** (`compress_1_pre`) | Trivial — replace inline `forall` with existing `bounded_i16_array (mk_i16 0) (mk_i16 3328)` atom. Five-minute fix. |
| 3 | **L6** (`cond_subtract_3329_post`) | Used at compress entry. Define `cond_subtract_3329_lane_post` opaque mirroring `compress_1_lane_post`. |
| 4 | **L9** (`to_unsigned_representative_post`) | Same shape as L6 — define `to_unsigned_lane_post` opaque. |
| 5 | **L7, L8** (`barrett_reduce_post`, `montgomery_multiply_by_constant_post`) | More involved — `mod_q_eq (...)` already opaque, but `v ... * v ... * 169` exposes Montgomery's `R^{-1} = 169` magic constant. Refactor would need a `mont_lane_post` opaque atom. |
| 6 | **`is_bounded_poly` opacity** (§2.3) | Mild leak only — tackle after L1–L9 because the migration touches more files. |
| 7 | **`lemma_bounded_i16_array_lookup` yield** (§1.3) | Wrap yield in a lane-form opaque atom. Smaller scope than the others. |

## Estimated effort

| Cleanup | Effort |
|---|---|
| Priority 1 (L1–L5, ~5 method pairs, single recipe) | 1–2 sessions; requires updating all backend impls + ~10 polynomial-level callers |
| Priority 2 (L10) | <1 hour |
| Priority 3 (L6) + Priority 4 (L9) | 1 session each |
| Priority 5 (L7, L8) | 1–2 sessions (Montgomery reasoning is delicate) |
| Priority 6 (`is_bounded_poly` opaque) | 1 session |
| Priority 7 (`lemma_bounded_i16_array_lookup` yield) | <1 day |

Total: **5–8 sessions** for the full boundary cleanup. The Priority 1
recipe is the biggest leverage — closing add/sub/negate/multiply seals
the most-invoked four primitive operations.

## Out-of-scope (already addressed elsewhere)

- The opaque `*_lane_post` / `*_branch_post` predicates (§1.2) and the
  `is_i16b_array_opaque` / `bounded_i16_array` family are already
  implemented correctly. Don't touch.
- The dual-pattern SMTPats in `polynomial.rs` (§2.1) are correctly
  designed — don't simplify or remove them.
- Intro/higher lemmas without SMTPats (§2.2) are correctly explicit-only —
  don't add SMTPats.
- The hacspec FE-equation specs cited in `compress_*_lane_post`,
  `decompress_*_lane_post`, `ntt_*_branch_post` are out of scope: they
  *are* the hacspec contract that the trait is meant to expose. Keeping
  them opaque (which they already are) is the correct discipline.

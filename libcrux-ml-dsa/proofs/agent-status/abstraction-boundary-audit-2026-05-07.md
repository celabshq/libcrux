# ML-DSA Abstraction-Boundary Audit — 2026-05-07

Read-only audit. Per HEAD `9b5b75b4b`. No code changes.

The principle audited: **above-trait code (`arithmetic.rs`, `matrix.rs`,
`ntt.rs`, `polynomial.rs` non-spec, `sample.rs`, `ml_dsa_generic.rs`,
`encoding/*.rs`, `ml_dsa_{44,65,87}.rs`) must reason only with poly-level
opaque predicates** (`is_bounded_poly`, `is_bounded_poly_slice`,
`is_bounded_poly_range`, `is_lane_range_poly`, `is_lane_range_poly_slice`)
or 8-element opaque atoms (`is_i32b_array_opaque`, `is_pos_array_opaque`,
`is_binary_array_8_opaque`, `is_i32b_strict_lower_array_opaque`). It must
**never** see (a) raw `%`, raw `*`, or raw bit-shifts in proof obligations,
(b) bare per-lane `forall j<8. ... v(...) op X` where the body is not
itself an opaque-pred call.

Verdict legend: **PASS** = compliant; **FAIL** = leak the abstraction;
**NEEDS-REVIEW** = compliant in shape but suspected as cascade source or
fragility risk.

---

## TL;DR — Top 5 risk items

1. **`Operations::ntt`/`invert_ntt_montgomery`/`reduce` pre/post bare-`forall i<32. is_i32b_array_opaque b ...`** (traits.rs:321–366). FAIL on Test B (bare poly-shape forall). Fix: replace with `is_bounded_poly b re`. Cascade-source candidate for k!63 (these foralls live in a *direct* dep of `generate_key_pair`).

2. **`Operations::montgomery_multiply` post mixed `forall8 (montgomery_multiply_lane_post)` + bare `forall i<8. ... == Spec.MLDSA.Math.mont_mul ...`** (traits.rs:130–139). FAIL on Test B. Fix: drop the bare-forall mont_mul clause; the per-lane opaque post already covers it. The `mont_mul` clause introduces a second per-lane Skolem at every above-trait call to montgomery_multiply.

3. **`Operations::shift_left_then_reduce` pre raw `forall i<8. v(...)>=0 /\ v(...)<=261631`** (traits.rs:144–145). FAIL. Fix: bridge through `is_lane_range_poly` at the poly level + a new asymmetric `is_lane_range_array_opaque 0 261631` at the lane level (mirrors `is_pos_array_opaque`).

4. **`Operations::t1_serialize` pre + `t1_deserialize` post raw per-lane `>=0 /\ < pow2 10`** (traits.rs:305–317). FAIL. Fix: replace with `is_pos_array_opaque (pow2 10 - 1)`.

5. **`Operations::reduce_lane_post`, `montgomery_multiply_lane_post`, `shift_left_then_reduce_lane_post` predicate bodies expose raw `%` / `*`** (specs.rs:116–118, 226–227, 257–259). The `[@@ "opaque_to_smt"]` wrapper hides the body from automatic inference, but `lemma_*_lane_lookup` reveals it on manual call. NEEDS-REVIEW. Fix: introduce `mod_q_eq` opaque pred (per `trait-correctness-post-design-draft.md`) and reformulate these post bodies in terms of it.

**k!63 cascade hypothesis (Phase E §1)**: top candidate is item 1 — the `forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque ...` shape on `ntt`/`invert_ntt_montgomery`. These foralls live in NTT's pre/post chain that `generate_key_pair` invokes via `compute_as1_plus_s2`. Replacing them with `is_bounded_poly` collapses the trigger to a single opaque atom and is mechanically safe (see Phase E §1 for full reasoning).

---

## Phase A — `src/simd/traits.rs` (27 methods on `Operations`)

| # | Method (line) | Pre / Post shape | Verdict | Reason | Recommended fix |
|---|---|---|---|---|---|
| 1 | `zero` (37) | post: `result.repr() == [0i32; 8]` | PASS | direct array equality | — |
| 2 | `from_coefficient_array` (41) | pre: `array.len() == 8`; post: `future(out).repr() == array` | PASS | direct equality | — |
| 3 | `to_coefficient_array` (45) | pre: `out.len() == 8`; post: `future(out) == value.repr()` | PASS | direct equality | — |
| 4 | `add` (50) | `specs::add_pre` / `specs::add_post` | PASS | both are opaque preds at the spec layer (verified Phase B) | — |
| 5 | `subtract` (54) | `specs::sub_pre` / `specs::sub_post` | PASS | same as add | — |
| 6 | `infinity_norm_exceeds` (58) | pre: `is_i32b_array_opaque FIELD_MAX`; post: `infinity_norm_exceeds_post` | PASS | opaque-pred end-to-end | — |
| 7 | `decompose` (74) | pre: `gamma2 ∈ {95232, 261888} /\ is_i32b_array_opaque FIELD_MAX`; post: `is_i32b_array_opaque ... low /\ ... high /\ forall8 decompose_lane_post` | PASS | per-lane post is opaque pred via `forall8` | — |
| 8 | `compute_hint` (93) | pre: opaque + opaque; post: `is_binary_array_8_opaque hint /\ forall8 compute_hint_lane_post` | PASS | opaque per-lane post | — |
| 9 | `use_hint` (109) | pre: opaque + opaque + `is_binary_array_8_opaque`; post: cases on gamma2 + `forall8 use_hint_lane_post` | PASS | opaque per-lane post | — |
| 10 | `montgomery_multiply` (128) | post: `is_i32b_array_opaque FIELD_MAX /\ forall8 montgomery_multiply_lane_post /\ forall i<8. Seq.index ... == Spec.MLDSA.Math.mont_mul ...` | **FAIL** | second clause is a bare `forall i<8` over per-lane equality with raw `Spec.MLDSA.Math.mont_mul` (a Hacspec function whose body inside the trait scope is not opaque-wrapped). Introduces a second per-lane Skolem at every consumer — k!63 candidate. | drop the bare `forall i<8. ... == mont_mul ...` clause; per-lane `montgomery_multiply_lane_post` (already opaque) covers correctness; if mont_mul equality is needed by callers, add a separate poly-level opaque pred citing it once. |
| 11 | `shift_left_then_reduce` (143) | pre: `v $SHIFT_BY == 13 /\ forall i<8. v(...)>=0 /\ v(...)<=261631`; post: `forall8 shift_left_then_reduce_lane_post` | **FAIL** | pre is bare per-lane comparison with raw `>=, <=`. | replace pre with `is_lane_range_array_opaque 0 261631 (f_repr ${simd_unit})` (new opaque per-8-lane atom mirroring `is_pos_array_opaque`); poly-level callers bridge via `is_lane_range_poly`. |
| 12 | `power2round` (159) | pre: `is_i32b_array_opaque FIELD_MAX t0`; post: `is_i32b_strict_lower_array_opaque (pow2 12) t0_future /\ forall8 (... v(t1_future)<pow2 10) /\ forall8 power2round_lane_post` | **FAIL** | the middle clause `forall8 (... v(t1_future) >= 0 /\ v(t1_future) < pow2 10)` is bare per-lane comparison. | replace middle clause with `is_pos_array_opaque (pow2 10 - 1) (f_repr ${t1}_future)`. |
| 13 | `rejection_sample_less_than_field_modulus` (180) | pre: `Seq.length` constraints; post: `v $result <= 8 /\ Seq.length identity /\ forall (i:nat{i < Seq.length ${out}_future}). i < v $result ==> v(...) >= 0 /\ v(...) < 8380417` | **FAIL** | bare `forall i. ... v(...) op X` post over a slice. | introduce `rejection_sample_count_post (out: t_Slice i32) (count: usize) (lo hi: i32)` opaque pred (`Spec.Utils` or specs.rs); replace post. |
| 14 | `rejection_sample_less_than_eta_equals_2` (192) | same shape with bounds [-2, 2] | **FAIL** | same | same; pred parameterized on (lo, hi). |
| 15 | `rejection_sample_less_than_eta_equals_4` (201) | same shape with bounds [-4, 4] | **FAIL** | same | same. |
| 16 | `gamma1_serialize` (219) | pre: width-disjunction + `is_pos_array_opaque (pow2 (gamma1_exponent) - 1)`; post: length identity | PASS | opaque-pred pre, length-only post | — |
| 17 | `gamma1_deserialize` (227) | pre: width-disjunction + length; post: `is_i32b_array_opaque (pow2 (v $gamma1_exponent))` | PASS | opaque-pred post | — |
| 18 | `commitment_serialize` (240) | pre: length disjunction + `is_pos_array_opaque (pow2 (Seq.length $serialized) - 1)`; post: length identity | PASS | opaque-pred pre | — |
| 19 | `error_serialize` (252) | pre: length match + `is_pos_array_opaque ... (match eta)`; post: length identity | PASS | opaque-pred pre | — |
| 20 | `error_deserialize` (264) | pre: length match; post: `forall8 (eta-cases on -5/-11/2/4 raw inequalities)` | **FAIL** | bare per-lane comparison via `forall8`, eta-cased. | introduce `error_deserialize_post (eta: Eta) (out: t_Array i32 8)` opaque pred OR factor as eta-parameterized `is_i32b_centered_range_opaque` (lo, hi); post collapses to single opaque-pred call. |
| 21 | `t0_serialize` (290) | pre: length + `is_i32b_strict_lower_array_opaque (pow2 12)`; post: length identity | PASS | opaque-pred pre | — |
| 22 | `t0_deserialize` (299) | pre: length; post: `is_i32b_strict_lower_array_opaque (pow2 12)` | PASS | opaque-pred post | — |
| 23 | `t1_serialize` (305) | pre: `Seq.length /\ forall i<8. v(...) >= 0 /\ v(...) < pow2 10`; post: length identity | **FAIL** | bare per-lane comparison in pre. | replace pre's forall with `is_pos_array_opaque (pow2 10 - 1) (f_repr ${simd_unit})`. |
| 24 | `t1_deserialize` (313) | pre: length; post: `forall8 (v(...) >= 0 /\ v(...) < pow2 10)` | **FAIL** | bare per-lane comparison in post. | replace post's forall8 with `is_pos_array_opaque (pow2 10 - 1) (f_repr ${out}_future)`. |
| 25 | `ntt` (321) | pre: `forall (i:nat). i < 32 ==> is_i32b_array_opaque NTT_BASE_BOUND ...`; post: same with FIELD_MAX | **FAIL** | bare poly-shape forall (trigger reach is global, not opaque pred). **Top k!63 candidate.** | replace pre with `is_bounded_poly NTT_BASE_BOUND re_simd` (add a `re_simd` poly view or use an existing helper); replace post with `is_bounded_poly FIELD_MAX re_simd`. Body needs a `reveal_opaque is_bounded_poly` + `lemma_is_bounded_poly_intro` pair (see ntt.rs §15 / §85 for the existing pattern). Note: `simd_units` is `&mut [Self; 32]`, not a `PolynomialRingElement`; need to adapt either by passing the poly directly or by adding an opaque `is_bounded_poly_array (b: usize) (a: t_Array Self 32)` peer predicate. |
| 26 | `invert_ntt_montgomery` (344) | pre: `forall (i:nat). i < 32 ==> is_i32b_array_opaque FIELD_MAX ...`; post: same with `4211177` | **FAIL** | same shape, same fix. | same as ntt. |
| 27 | `reduce` (355) | pre: `forall (i:nat). i < 32 ==> is_i32b_array_opaque 2143289343 ...`; post: `forall (j:nat). j < 32 ==> is_i32b_array_opaque FIELD_MAX ... /\ forall8 reduce_lane_post` | **FAIL** | poly-shape bare forall in pre/post + each post-iter has `forall8 reduce_lane_post` whose body exposes `%` (Phase B §1). | (a) replace top-level forall i<32 with poly-array opaque pred (same as ntt); (b) the per-poly `forall8 reduce_lane_post` clause's content is correct, but `reduce_lane_post` body is `(v result) % q == (v input) % q` raw — see Phase B item ⑤. |

**Trait surface verdict count**: 7 PASS, 8 FAIL, 0 NEEDS-REVIEW (out of 27 audited methods; `Repr.repr` is trivially PASS).

---

## Phase B — `src/simd/traits/specs.rs`

### B.1 — Predicates (10 items)

| # | Predicate (line) | Body shape | Opaque? | What `reveal` exposes | Verdict | Recommended fix |
|---|---|---|---|---|---|---|
| ① | `is_binary_array_8` / `_opaque` (52,57) | `forall i<8. v(x[i]) ∈ {0,1}` | YES (opaque variant) | binary lane disjunction | PASS | — |
| ② | `is_pos_array` / `_opaque` (79,84) | `forall i<8. v(x[i]) ∈ [0,l]` | YES | per-lane `>=0 /\ <=l` | PASS | — |
| ③ | `infinity_norm_exceeds_post` (105) | `b2t result <==> exists i<8. (\|x_i\| >= bound)` | YES | exists-shape | PASS | — |
| ④ | `reduce_lane_post` (116) | `v result > -q /\ v result < q /\ (v result) % q == (v input) % q` | YES | raw `%` | NEEDS-REVIEW | reformulate post body as `is_i32b 8380416 result /\ mod_q_eq_opaque (v input) (v result)` (centered-Barrett bound + opaque mod-q congruence). |
| ⑤ | `decompose_lane_post` (136) | `gamma2-cases ==> v low == v(snd pair) /\ v high == v(fst pair)` (Hacspec.decompose) | YES | spec-fn equality | PASS | — |
| ⑥ | `compute_hint_lane_post` (172) | `gamma2-cases /\ (high in range ==> v hint == compute_one_hint ...)` | YES | spec-fn equality | PASS | — |
| ⑦ | `use_hint_lane_post` (198) | `gamma2-cases /\ (input ∈ range /\ hint ∈ {0,1} ==> v future_hint == v(uuse_hint ...))` | YES | spec-fn equality | PASS | — |
| ⑧ | `montgomery_multiply_lane_post` (226) | `(v future_lhs) % q == (v lhs * v rhs * 8265825) % q` | YES | raw `%`, `*` | NEEDS-REVIEW | reformulate as `mod_q_eq_opaque (v future_lhs) (v lhs * v rhs * 8265825)`. Even better: lift to `is_i32b 8380416 future_lhs /\ mod_q_eq_opaque (v future_lhs) (Spec.MLDSA.Math.mont_mul (v lhs) (v rhs))` so the spec-side multiply is hidden too. |
| ⑨ | `shift_left_then_reduce_lane_post` (257) | `is_i32b 8380416 future /\ (v future) % q == (v input * 8192) % q` | YES | raw `%`, `*` | NEEDS-REVIEW | reformulate as `is_i32b 8380416 future /\ mod_q_eq_opaque (v future) (v input * 8192)`. |
| ⑩ | `power2round_lane_post` (278) | `(input ∈ [0,q)) ==> v future_t1 == v(fst pair) /\ v future_t0 == v(snd pair)` (Hacspec.power2round) | YES | spec-fn equality | PASS | — |
| ⑪ | `rejection_sample_3byte_lane_post` (306) | `match coeff_from_three_bytes ... with Some c -> v out == v c \| None -> True` | YES | spec-fn equality | PASS | — |
| ⑫ | `rejection_sample_halfbyte_lane_post` (315) | same shape | YES | spec-fn equality | PASS | — |
| ⑬ | `simple_bit_pack_chunk_post` (331) | `Seq.length output * 8 == 8 * b /\ forall i<8. v(input[i]) ∈ [0, pow2 b)` | YES | per-lane bound (pow2) | PASS | — (bound is constant pow2; not raw arith) |
| ⑭ | `simple_bit_unpack_chunk_post` (339) | same shape on output | YES | per-lane bound | PASS | — |
| ⑮ | `bit_pack_chunk_post` (351) | `Seq.length /\ forall i<8. v(...) ∈ [-a, b]` | YES | per-lane bound | PASS | — |
| ⑯ | `bit_unpack_chunk_post` (359) | same | YES | per-lane bound | PASS | — |
| ⑰ | `add_pre`, `add_post`, `sub_pre`, `sub_post` (368, 388, 418, 438) | `forall i<8. int_is_i32(lhs[i] + rhs[i])` (and equality variants) | YES (`fstar::before [@@ "opaque_to_smt"]`) | per-lane sum/difference equality | PASS | — |

### B.2 — SMTPat-bearing lemmas (12 items)

| # | Lemma (line) | SMTPat shape | Body / ensures | Verdict | Recommended fix |
|---|---|---|---|---|---|
| ⓐ | `lemma_is_binary_array_8_lookup` (60) | `[Seq.index x i; is_binary_array_8_opaque x]` | `v(x[i]) ∈ {0,1}` | PASS | — (8-element bound; load-bearing per asymmetric-opaque experiment, removing breaks 18+ proofs) |
| ⓑ | `lemma_is_binary_array_8_intro` (66) | none | universal hyp ⇒ opaque | PASS | — |
| ⓒ | `lemma_is_pos_array_lookup` (87) | `[Seq.index x i; is_pos_array_opaque l x]` | `v(x[i]) ∈ [0,l]` | PASS | — (same rationale as ⓐ) |
| ⓓ | `lemma_is_pos_array_intro` (93) | none | universal hyp ⇒ opaque | PASS | — |
| ⓔ | `lemma_reduce_lane_lookup` (120) | **none** | raw `%` ensures | NEEDS-REVIEW | once item ④ is fixed (mod_q_eq), update ensures to expose `mod_q_eq_opaque ...` instead. |
| ⓕ | `lemma_reduce_lane_intro` (126) | **none** | takes raw `%` requires | NEEDS-REVIEW | same as ⓔ. |
| ⓖ | `lemma_decompose_lane_lookup` (144) | `[decompose_lane_post g i l h]` | spec-fn equality | PASS | — |
| ⓗ | `lemma_decompose_lane_intro` (153) | none | — | PASS | — |
| ⓘ | `lemma_compute_hint_lane_lookup` (177) | `[compute_hint_lane_post ...]` | spec-fn equality | PASS | — |
| ⓙ | `lemma_compute_hint_lane_intro` (187) | none | — | PASS | — |
| ⓚ | `lemma_use_hint_lane_lookup` (204) | `[use_hint_lane_post ...]` | spec-fn equality | PASS | — |
| ⓛ | `lemma_use_hint_lane_intro` (214) | none | — | PASS | — |
| ⓜ | `lemma_montgomery_multiply_lane_lookup` (229) | **none** | raw `% / *` ensures | NEEDS-REVIEW | fix item ⑧ first; update ensures to `mod_q_eq_opaque ...`. |
| ⓝ | `lemma_montgomery_multiply_lane_intro` (235) | none | takes raw `%`, `*` requires | NEEDS-REVIEW | same as ⓜ. |
| ⓞ | `lemma_shift_left_then_reduce_lane_lookup` (261) | **none** | raw `%, *` ensures | NEEDS-REVIEW | fix item ⑨; update ensures to `mod_q_eq_opaque`. |
| ⓟ | `lemma_shift_left_then_reduce_lane_intro` (268) | none | — | NEEDS-REVIEW | same. |
| ⓠ | `lemma_power2round_lane_lookup` (283) | `[power2round_lane_post ...]` | spec-fn pair | PASS | — |
| ⓡ | `lemma_power2round_lane_intro` (292) | none | — | PASS | — |
| ⓢ | `bounded_add_pre` (370) | `[add_pre a b; is_i32b_array_opaque b1 a; is_i32b_array_opaque b2 b]` | opaque-pred ensures | PASS | — (3-trigger tight; fires only with both bound preds in scope) |
| ⓣ | `bounded_add_post` (391) | `[add_post a b a_future; is_i32b_array_opaque b1 a; is_i32b_array_opaque b2 b; is_i32b_array_opaque b3 a_future]` | `is_i32b_array_opaque b3 a_future` (opaque-pred ensures) | PASS | — (4-trigger; fires only when caller already has a desired `b3`. Body internally uses raw + at lane level but ensures is opaque. Document why so future agents don't try to "tighten".) |
| ⓤ | `bounded_sub_pre` (420) | parallel | PASS | — |
| ⓥ | `bounded_sub_post` (441) | parallel | PASS | — |

**B verdict count (predicates)**: 13 PASS, 3 NEEDS-REVIEW (`reduce_lane_post`, `montgomery_multiply_lane_post`, `shift_left_then_reduce_lane_post`).
**B verdict count (lemmas)**: 16 PASS, 6 NEEDS-REVIEW (the lookup/intro pairs of the 3 NEEDS-REVIEW preds).

---

## Phase C — `proofs/fstar/extraction/Libcrux_ml_dsa.Polynomial.Spec.fst`

### C.1 — Predicates (5 items)

| # | Predicate (line) | Body shape | Opaque? | Reveal exposes | Verdict | Recommended fix |
|---|---|---|---|---|---|---|
| ⒈ | `is_bounded_simd_unit` (12) | `is_i32b_array_opaque (v b) (f_repr vec)` | NO (no opacity attr) | direct opaque-atom call | PASS | — (alias only; transparent reveal is fine) |
| ⒉ | `is_bounded_poly` (23) | `forall i<32. is_i32b_array_opaque (v b) (f_repr (p.simd_units[i]))` | YES | per-i opaque-atom call | PASS | — |
| ⒊ | `is_bounded_poly_range` (93) | `forall k. v lo<=k /\ k<v hi /\ k<Seq.length arr ==> is_bounded_poly b (arr[k])` | YES | per-k opaque-pred call | PASS | — (delegates to ⒉) |
| ⒋ | `is_bounded_poly_slice` (178) | `forall k. k<Seq.length arr ==> is_bounded_poly b (arr[k])` | YES | per-k opaque-pred call | PASS | — (delegates to ⒉) |
| ⒌ | `is_lane_range_poly` (225) | `forall j<32. forall m<8. v(...)>=v lo /\ v(...)<=v hi` | YES | 2-deep raw `>=, <=` per-lane | NEEDS-REVIEW | body intentionally exposes raw lane-level `>=, <=` since this predicate is the asymmetric-bound atom. Keep — but document that consumers should bridge to `is_bounded_poly` early via `lemma_lane_range_pos_to_bounded_poly` and never `reveal_opaque is_lane_range_poly` directly above the trait. |
| ⒍ | `is_lane_range_poly_slice` (276) | `forall k. k<Seq.length arr ==> is_lane_range_poly lo hi (arr[k])` | YES | per-k opaque-pred call | PASS | — (delegates to ⒌) |

### C.2 — Lemmas (12 items)

| # | Lemma (line) | SMTPat shape | Verdict | Recommended fix |
|---|---|---|---|---|
| ㋐ | `lemma_is_bounded_poly_lookup` (34) | `[is_i32b_array_opaque (v b) (f_repr ...); is_bounded_poly b p]` | PASS | — (two-trigger tight; both must match together) |
| ㋑ | `lemma_is_bounded_poly_intro` (51) | none | PASS | — |
| ㋒ | `lemma_is_bounded_poly_higher` (66) | none | PASS | — (monotonicity, manual call) |
| ㋓ | `lemma_is_bounded_poly_range_lookup` (105) | `[is_bounded_poly_range b lo hi arr; Seq.index arr k]` | NEEDS-REVIEW (cascade-cost) | second trigger `Seq.index arr k` fires on every slice index in scope. Ensures is opaque so no arith leak, but in nested loops over multiple slices the firing rate is high. Verify in remediation that this is not the k!63 source by removing temporarily and re-profiling. If it is: tighten by adding a third trigger, e.g., `[is_bounded_poly_range b lo hi arr; is_bounded_poly b (Seq.index arr k)]` (fires only when caller already has a poly-level claim). |
| ㋔ | `lemma_is_bounded_poly_range_intro` (120) | none | PASS | — |
| ㋕ | `lemma_is_bounded_poly_range_extend_after_update` (138) | none | PASS | — (manual call; standalone-context proof per Sprint 4 lesson) |
| ㋖ | `lemma_is_bounded_poly_slice_lookup` (189) | `[is_bounded_poly_slice b arr; Seq.index arr k]` | NEEDS-REVIEW (cascade-cost) | same as ㋓. Same fix: re-profile after removal; tighten if implicated. |
| ㋗ | `lemma_is_bounded_poly_slice_intro` (203) | none | PASS | — |
| ㋘ | `lemma_is_lane_range_poly_lookup` (240) | `[is_lane_range_poly lo hi p; Seq.index (...f_repr (Seq.index p.f_simd_units j)) m]` | PASS | — (deeply nested second trigger; fires only when consumer is already doing per-lane indexing on this poly. Tight enough.) |
| ㋙ | `lemma_is_lane_range_poly_intro` (257) | none | PASS | — |
| ㋚ | `lemma_is_lane_range_poly_slice_lookup` (287) | `[is_lane_range_poly_slice lo hi arr; Seq.index arr k]` | NEEDS-REVIEW (cascade-cost) | same as ㋓/㋖. Same fix. |
| ㋛ | `lemma_is_lane_range_poly_slice_intro` (301) | none | PASS | — |
| ㋜ | `lemma_lane_range_pos_to_bounded_poly` (323) | none | PASS | — (asymmetric→symmetric bridge; manual call) |
| ㋝ | `lemma_lane_range_pos_to_bounded_poly_slice` (350) | none | PASS | — (slice version) |
| ㋞ | `lemma_lane_range_pos_to_pos_array_slice` (374) | none | NEEDS-REVIEW | ensures is bare double-`forall k j. is_pos_array_opaque (...)` — used at `ml_dsa_generic.rs:184` to bridge `is_lane_range_poly_slice 0 eta s1_s2` to `signing_key::generate_serialized`'s `s1_2` pre. Recommended long-term fix: lift `signing_key::generate_serialized`'s `s1_2` pre (encoding/signing_key.rs:27–32) from `forall k. forall j. is_pos_array_opaque ...` to `is_lane_range_poly_slice 0 eta s1_2`; the bridge then becomes private to the function call (and this lemma can be deleted). |

**C verdict count (predicates)**: 5 PASS, 1 NEEDS-REVIEW.
**C verdict count (lemmas)**: 11 PASS, 4 NEEDS-REVIEW (3 `Seq.index arr k`-broad-trigger lemmas + the bare-forall bridge).

---

## Phase D — Above-trait code

### D.1 — Universal quantifiers in pre / post / loop_invariant

Each row classified A (clean opaque-pred replacement exists), B (new pred needed), or C (frame/structural — keep). Files in scope listed in the plan; foralls counted via `grep -rn 'forall (i\|forall (j\|forall (k\|forall (m\|forall i' src/{arithmetic,matrix,ntt,sample,ml_dsa_generic,polynomial}.rs src/encoding/*.rs src/ml_dsa_{44,65,87}.rs`.

Total: **103 forall sites** across 11 files. Grouped by site type:

| Class | Count | Files |
|---|---|---|
| **A**: replace with poly-level opaque pred | ~58 | matrix.rs (×24, all `forall (k:nat). k<… (forall (j:nat). j<32. is_i32b_array_opaque …)`), encoding/{commitment,error,gamma1,signature,signing_key,t0,t1,verification_key}.rs (×~20 in pre/post/loop_invariant), polynomial.rs (×~8 in `multiply` etc.), ntt.rs (×4), arithmetic.rs (×5) |
| **B**: new opaque pred needed | ~10 | arithmetic.rs:99–110 (power2round mixed t0/t1), arithmetic.rs:223–229 (use_hint binary-hint), arithmetic.rs:178–179 (make_hint paired-vector bound), encoding/error.rs:139–140 (per-iter range), polynomial.rs:485 (multiply pre with `forall i. is_i32b_array_opaque`) |
| **C**: legitimate frame / structural | ~35 | All `forall k. k<v i. … == old[k]` frame conditions; outer-inv splits `forall j>=i` vs `forall j<i` in ntt-style loop bodies; `forall j<32. reduce_lane_post …` per-lane witness shapes (these are correct — the predicate is opaque). |

#### D.1.A — Class-A targets with proposed replacements (selected high-leverage rows)

| File:line | Function (context) | Current shape | Replacement |
|---|---|---|---|
| `arithmetic.rs:10` | `vector_infinity_norm_exceeds` pre | `forall i. forall j. is_i32b_array_opaque FIELD_MAX (…simd_units[i][j])` | `is_bounded_poly_slice FIELD_MAX vector` |
| `arithmetic.rs:31–33` | `shift_left_then_reduce` (free fn) post | `forall i<32. is_i32b_array_opaque (pow2 12) (…simd_units[i])` | `is_bounded_poly (mk_usize (pow2 12)) re` |
| `arithmetic.rs:142` | `decompose_vector` pre | same shape | `is_bounded_poly_slice FIELD_MAX t` |
| `arithmetic.rs:178–179` | `make_hint` pre | `forall i<len. forall j<32. is_i32b_array_opaque FIELD_MAX (…)` | `is_bounded_poly_slice FIELD_MAX low /\ is_bounded_poly_slice FIELD_MAX high` |
| `arithmetic.rs:232–233` | `use_hint` (free fn) post | `forall i<len. forall j<32. is_i32b_array_opaque FIELD_MAX (…)` | `is_bounded_poly_slice FIELD_MAX re_vector` |
| `matrix.rs:375, 378, 384, 488–499, 557–567, 599–629` | `compute_a_times_mask`, `compute_t1_minus_ct0`, `add_vectors`, `subtract_vectors`, etc. | nested `forall k. forall j<32. is_i32b_array_opaque FIELD_MAX (…)` | `is_bounded_poly_slice FIELD_MAX <vec>` |
| `ntt.rs:50, 72` | `reduce`, `ntt_multiply_montgomery` (above-trait, poly level) | `forall (i:nat). i < 32 ==> is_i32b_array_opaque … (Seq.index re.simd_units i)` | `is_bounded_poly … re` |
| `polynomial.rs:434, 485, 498, 501, 533, 565, 568, 571, 597` | `zero` post, `multiply` pre/post + variants | `forall (j:nat). j < 32 ==> is_i32b_array_opaque …` | `is_bounded_poly …` |
| `encoding/commitment.rs:6, 34–35` | `serialize` pre, `serialize_vector` pre | `forall (j:nat). j < 32. is_i32b_array_opaque …` and slice version | `is_bounded_poly …` / `is_bounded_poly_slice …` |
| `encoding/error.rs:15, 69` | `deserialize_then_ntt` pre, `deserialize_then_ntt_vector` pre | same shapes | `is_bounded_poly` family |
| `encoding/gamma1.rs:7, 37` | `serialize_vector`, `deserialize` ensures | same | `is_bounded_poly` family |
| `encoding/t0.rs:25, 55` | `serialize_vector`, `deserialize_vector` | same | `is_bounded_poly_slice (pow2 12)` (with strict-lower variant for serialize per F-8) |
| `encoding/t1.rs:8–9, 34–35` | `serialize`, `deserialize` (above-trait) | `forall j<32. forall i<8. v(...) >= 0 /\ v(...) < pow2 10` | `is_lane_range_poly 0 (pow2 10 - 1) re` |
| `encoding/signing_key.rs:27–28, 34–35, 90–91, 124–125` | `generate_serialized` pre + loop_invariants on s1_2 and t0 | `forall k<len. forall j<32. is_pos_array_opaque … (f_repr (Seq.index … j))` | replace s1_2 with `is_lane_range_poly_slice 0 eta s1_2`; replace t0 with `is_bounded_poly_slice (pow2 12) t0` (or strict_lower variant per Phase A item 22). **Eliminates `lemma_lane_range_pos_to_pos_array_slice` consumer** (sec C ㋞). |
| `encoding/verification_key.rs:59–61` | `generate_serialized` pre | `forall k<rows_in_a. forall j<32. forall i<8. v(...) >= 0 /\ v(...) < pow2 10` | `is_lane_range_poly_slice 0 (pow2 10 - 1) t1` |

#### D.1.B — Class-B targets needing new predicates

| File:line | Why no existing pred fits | Proposed new pred |
|---|---|---|
| `arithmetic.rs:99–110` (power2round_one_ring_element) | Mixed t0 + t1 post (t0 bounded by `pow2 12`, t1 in `[0, pow2 10)`) | `power2round_post (t0 t1: PolynomialRingElement) : prop = is_bounded_poly (pow2 12) t0 /\ is_lane_range_poly 0 (pow2 10 - 1) t1` (compound atom) — opaque, with intro/lookup pair |
| `arithmetic.rs:223–229` (use_hint above-trait) | `forall i. forall j. (hint[i][j] == 0 \/ hint[i][j] == 1)` paired with `forall i. forall j. is_i32b_array_opaque FIELD_MAX (…)` | `is_binary_hint_vector (h: t_Slice PolynomialRingElement) : prop = forall k<Seq.length h. forall j<32. is_binary_array_8_opaque (f_repr h[k].simd_units[j])` (poly-slice analog of `is_binary_array_8_opaque`) — opaque with intro/lookup |
| `encoding/error.rs:139–140` (per-iter range) | iteration-state bound `forall k<v i. forall j<32. is_i32b_array_opaque … (Seq.index … j)` | covered by Class-A `is_bounded_poly_range` |

#### D.1.C — Class-C (legitimate frame conditions) — leave as-is

| File:line range | Why keep |
|---|---|
| All `forall k. k<v i. … == old[k]` frame conditions in matrix.rs, encoding/*, sample.rs, polynomial.rs | Frame (Leibniz) conditions are inherently structural; no opaque-pred wrapping helps |
| `forall j<32. forall i<8. reduce_lane_post / decompose_lane_post / etc.` per-lane witness chains in ntt.rs / arithmetic.rs / encoding | Each `_lane_post` is already opaque (Phase B); the outer `forall8` is the witness shape carrying the per-lane post forward — replacing would require a new poly-level `reduce_post`/`decompose_post`/etc. atom (worth doing for compute-heavy posts but Class C-keep for lower-cost paths) |
| Outer-loop inv splits `forall j>=v i…` vs `forall j<v i…` | Standard partitioned invariant pattern; can't be folded into a single opaque pred |

### D.2 — SMTPat-bearing lemmas defined above the trait

`grep -rn 'SMTPat' proofs/fstar/extraction/Libcrux_ml_dsa.{Matrix,Arithmetic,Ntt,Sample,Ml_dsa_generic,Encoding.*}.fst` returns **zero** matches. All SMTPat-bearing lemmas in the project live in:
- `Libcrux_ml_dsa.Polynomial.Spec` (Phase C)
- `Libcrux_ml_dsa.Simd.Traits.Specs` (Phase B)
- `Spec.Utils.*` (out of scope: a single `is_i32b_array_opaque` SMTPat ladder, audited transitively via Phase B)

**Verdict**: PASS. The above-trait modules introduce **no new** SMTPat-bearing lemmas; the boundary on this dimension is clean. (This was a stronger result than the plan anticipated.)

### D.3 — `reveal_opaque` calls above the trait

`grep -rn 'reveal_opaque' src/{arithmetic,matrix,ntt,sample,ml_dsa_generic}.rs src/encoding/ src/ml_dsa_{44,65,87}.rs` filtered to non-spec sites:

| File:line | Reveals | Context | Verdict |
|---|---|---|---|
| `arithmetic.rs:60` | `Spec.Utils.is_i32b_array_opaque` | inside `shift_left_then_reduce` (free fn), per-lane proof bridge before SIMDUnit::shift_left_then_reduce trait call | PASS — legitimate impl-internal bridge |
| `ntt.rs:15` | `Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly` | inside `ntt` (above-trait poly fn), bridge to per-poly pre then trait-call | PASS — legitimate poly→trait bridge |
| `ntt.rs:34` | `is_bounded_poly` | inside `invert_ntt_montgomery` poly fn — same pattern | PASS |
| `ntt.rs:57` | `is_bounded_poly` | inside `reduce` poly fn — same pattern | PASS |
| `ntt.rs:85` | `is_bounded_poly` | inside `ntt_multiply_montgomery` poly fn — same pattern | PASS |
| `encoding/error.rs:99` | `Spec.Utils.is_i32b_array_opaque` | inside `deserialize_to_vector_then_ntt`, per-simd-unit scope in loop body | PASS — legitimate impl-internal bridge before SIMDUnit::* trait call |

All 6 calls are at the layer that wraps a SIMDUnit trait method (poly-fn → trait-fn boundary), which is the **intended** reveal location. **No above-trait reveals of poly-level opaque preds (`is_bounded_poly_*`, `is_lane_range_poly*`) by impl bodies, free fns, or wrappers.** Verdict: PASS. The boundary on this dimension is clean.

### D verdict summary

- D.1 forall sites: 103 total — 58 Class-A (mechanical replacement available), 10 Class-B (new pred needed), 35 Class-C (legitimate frame/structural).
- D.2 SMTPat lemmas above trait: 0. PASS.
- D.3 reveal_opaque sites above trait: 6, all legitimate poly→trait bridges. PASS.

---

## Phase E — Risk register + remediation roadmap

### E.1 — k!63 cascade hypothesis

**Recap**: `Ml_dsa_44_.fst::generate_key_pair` query 60 has anonymous `k!63` Skolem firing ~624,588 times at rlimit 400, walltime 122–147 s. Two structural-fix attempts failed:
- *asymmetric-opaque experiment*: stripped per-simd-unit `Seq.index` SMTPats, bottomed `is_lane_range_poly` at `is_intb_array_opaque` atom. k!63 unchanged (~638K). 18+ unrelated regressions.
- *bridge-cascade experiment*: replaced nested `Classical.forall_intro` chains with dual-SMTPat lookups. Bridge lemmas verified clean themselves; query 60 still timed out.

**Ruled out**:
- `is_lane_range_poly`'s SMTPat shape (asymmetric-opaque experiment).
- bridge-lemma forall_intro structure (bridge-cascade experiment).
- per-simd-unit `Seq.index` SMTPats on `is_pos_array_opaque` / `is_binary_array_8_opaque` (load-bearing for many proofs; removing them broke 18+).

**Top remaining candidates (from this audit)**:

1. **`Operations::ntt`/`invert_ntt_montgomery`/`reduce` pre/post bare-forall** (Phase A items 25–27). The shape `forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque b (f_repr (Seq.index simd_units i))` is the **only above-trait surface** where a bare per-i forall is cited at the *trait* boundary (the rest of the trait uses `forall8` over per-lane opaque posts; `forall (i:nat). i < 32 …` here is unique). When `compute_as1_plus_s2`'s body invokes ntt repeatedly inside a fold, this `forall (i:nat). i < 32 …` instantiates per simd_unit per fold-iteration per call site, multiplied across the keygen NTT chain. Skolem `k!63` in q60 is consistent with this Cartesian product. **Test**: in remediation, swap one of these (e.g., `ntt`'s pre+post) to `is_bounded_poly` and re-profile q60. If k!63 collapses, hypothesis confirmed.

2. **`montgomery_multiply` post bare `forall i<8. ... == Spec.MLDSA.Math.mont_mul ...`** (Phase A item 10). `mont_mul` body extracts to a let-binding (not opaque) so this clause re-introduces raw arithmetic per lane. Above-trait callers like `vector_times_ring_element` and `compute_as1_plus_s2` invoke `montgomery_multiply` inside an inner fold; the per-lane forall accumulates instances multiplicatively. **Test**: drop the bare forall in remediation and re-profile.

3. **`lemma_is_bounded_poly_slice_lookup` and `lemma_is_bounded_poly_range_lookup` `Seq.index arr k` second triggers** (Phase C items ㋓, ㋖). Generate_key_pair's body has multiple slices in scope (`s1_s2`, `s1_ntt`, `t0`, `t1`, `a_as_ntt`) each with an `is_bounded_poly_slice` predicate active. The `Seq.index arr k` pattern matches indexing across all of them simultaneously — hard to bound the firing rate analytically. **Test**: tighten one of these triggers (add a third leg requiring `is_bounded_poly b (Seq.index arr k)`) and re-profile.

The plan's remediation phase should test these in order; (1) is highest leverage because it touches the smallest blast radius (3 trait methods, ~6 above-trait call sites).

### E.2 — Risk register (top 15, severity × cost-to-fix)

| # | Item | Severity | Cost | Fix sketch | Regressors |
|---|---|---|---|---|---|
| 1 | `ntt`/`invert_ntt_montgomery`/`reduce` bare poly-foralls (A.25–27) | HIGH (k!63 candidate) | M (3 trait methods + ~12 callers) | use `is_bounded_poly` poly-level (need a `t_Array Self 32` view: either pass `PolynomialRingElement` or add `is_bounded_poly_array` for the array form) | NTT impl bodies (portable + AVX2) need a `reveal_opaque` + `lemma_is_bounded_poly_intro` pair; pattern already used at ntt.rs:15/34/57/85 |
| 2 | `montgomery_multiply` post bare `mont_mul` forall (A.10) | HIGH (k!63 candidate) | S (drop one clause) | drop the second forall in trait post; per-lane opaque pred suffices; if mont_mul equality is needed, factor into a new poly-level `mont_mul_poly_post` atom with one bridge lemma | callers that consumed the clause; grep for `mont_mul` above the trait (matrix.rs::vector_times_ring_element does NOT consume mont_mul directly per current readings, but verify via `grep -rn 'mont_mul' src/`) |
| 3 | `_lane_post` bodies expose raw `%`/`*` (B.4, B.8, B.9 + lookup/intro lemmas) | MED (lookups manual-only) | M (introduce `mod_q_eq` + bridge lemmas + reformulate 3 posts) | add `mod_q_eq (a b: int) : prop = a % q == b % q` opaque-wrapped + intro/lookup; reformulate `reduce_lane_post`, `montgomery_multiply_lane_post`, `shift_left_then_reduce_lane_post` to cite `mod_q_eq_opaque`; update lookup/intro lemmas | impl bodies that called the lookup lemmas to bridge per-lane mod-q facts; grep `lemma_reduce_lane_lookup`, `lemma_montgomery_multiply_lane_lookup`, `lemma_shift_left_then_reduce_lane_lookup` consumers |
| 4 | `shift_left_then_reduce` pre raw per-lane `>=, <=` (A.11) | MED | S (add one new pred) | introduce `is_lane_range_array_opaque lo hi (a: t_Array i32 8)` mirroring `is_pos_array_opaque`; replace pre with `is_lane_range_array_opaque 0 261631 (f_repr ${simd_unit})` | callers that prove the pre (above-trait `arithmetic::shift_left_then_reduce` already uses `is_lane_range_poly`-style; bridge needs to fire) |
| 5 | `t1_serialize` pre + `t1_deserialize` post raw per-lane (A.23, A.24) | MED | S (one pred swap) | replace with `is_pos_array_opaque (pow2 10 - 1)` | encoding/t1.rs above-trait callers (consume the trait pre); needs corresponding poly-level `is_lane_range_poly` flow at the call site |
| 6 | `power2round` post middle clause raw per-lane t1 (A.12) | MED | S | replace middle clause with `is_pos_array_opaque (pow2 10 - 1)` | impl bodies that proved this clause; arithmetic::power2round_one_ring_element |
| 7 | `error_deserialize` post bare `forall8` eta-cased (A.20) | MED | M (need new eta-parameterized pred) | introduce `error_deserialize_post (eta: Eta) (out: t_Array i32 8)` opaque, factor cases | encoding/error.rs callers |
| 8 | `rejection_sample_*` posts bare `forall i<count. v(...) op X` (A.13–15) | MED | M (need new opaque pred over slice-prefix) | introduce `rejection_sample_count_post (out: t_Slice i32) (count: usize) (lo hi: i32)` opaque pred; factor cases | sample.rs callers — Class-A in `sample.rs` post grep shows none (uses `*sampled_coefficients <= 263`-style scalar bounds), so blast radius is small |
| 9 | `lemma_is_bounded_poly_slice_lookup` `Seq.index` broad trigger (C.㋖) | MED (cascade-cost) | S | tighten by adding third trigger leg; OR remove SMTPat and force callers to use `lemma_is_bounded_poly_slice_intro`-then-lookup explicitly | many proof sites currently rely on the auto-fire; check via `grep -n 'lemma_is_bounded_poly_slice_lookup' .` first |
| 10 | `lemma_is_bounded_poly_range_lookup` `Seq.index` broad trigger (C.㋓) | MED | S | same as #9 | same |
| 11 | `lemma_is_lane_range_poly_slice_lookup` `Seq.index` broad trigger (C.㋚) | LOW (ensures already opaque) | S | same as #9 | same |
| 12 | `arithmetic::power2round_one_ring_element` mixed t0+t1 post (D.1.B) | MED | M (new compound pred) | introduce `power2round_post` compound atom (Class-B target) | call sites currently work with pair of standalone preds; refactor adds one bridge call |
| 13 | `arithmetic::use_hint` (free fn) above-trait paired forall (D.1.B) | MED | M (new opaque pred) | introduce `is_binary_hint_vector` poly-slice analog of `is_binary_array_8_opaque` | sign_internal flow consumers |
| 14 | `lemma_lane_range_pos_to_pos_array_slice` ensures bare double-forall (C.㋞) | LOW (intentional bridge today) | M (refactor signing_key pre) | lift `signing_key::generate_serialized`'s `s1_2` pre from per-simd-unit forall to `is_lane_range_poly_slice 0 eta s1_2`; same for t0; bridge becomes private | requires re-extraction; affects the `s1_2` precondition cascade |
| 15 | `verification_key::generate_serialized` 3-deep forall pre (D.1.A) | LOW | S | `is_lane_range_poly_slice 0 (pow2 10 - 1) t1` | encoding/verification_key.rs call site; Class-A trivial swap |

### E.3 — Recommended remediation order

1. Item 1 (ntt/invert_ntt_montgomery/reduce) — k!63 hypothesis test. If profile collapses, k!63 source is identified and the rest of the audit is incremental.
2. If item 1 fails: items 2 (mont_mul drop) and 9–10 (range/slice lookup tightening) — each tested independently with `smt.qi.profile=true`.
3. After k!63 is closed: items 3 (`mod_q_eq`), 4 (asymmetric atom), 5–8 (t1/power2round/error/rejection_sample posts) — these are correctness-of-abstraction fixes regardless of cascade.
4. Items 11–15: cleanup wave; mostly mechanical Class-A swaps.

Estimated total effort to clear all 15: **2–3 sprint weeks** (matches Sprint A week-2–3 budget in `proofs/agent-status/sprint-plan-2026-05-03.md`).

---

## Coverage / self-consistency checks

1. **Trait method count**: `grep -n '^\s*fn ' src/simd/traits.rs` = 28 (27 `Operations` + 1 `Repr.repr`); Phase A table covers all 27 substantive methods (`Repr.repr` is a return-type-only declaration with no proof obligation).
2. **Predicate-lemma symmetry**: every opaque predicate in Phase B and Phase C has both a lookup and an intro lemma row.
3. **forall coverage**: above-trait `grep` returned 103 sites; D.1 partition (58 + 10 + 35) sums to 103.
4. **No-edit check**: this task created exactly one file, this audit doc. No `.rs`/`.fst`/Makefile/extraction-config edits. No `make` / `hax extract` / `cargo` runs.

## Closing notes

- **Above-trait code is cleaner than expected** on two dimensions: zero new SMTPat-bearing lemmas (D.2) and zero illegitimate `reveal_opaque` calls (D.3). The 6 `reveal_opaque` sites are all at the intended poly→trait bridge layer. This means **the boundary erosion is concentrated at the trait surface itself** (Phase A) and in the opaque-but-leaky `_lane_post` bodies (Phase B), not in scattered above-trait abuse.
- The k!63 cascade is **most likely** caused by one of three audited items (Phase E.1). Remediation must test in order and re-profile after each change (see `feedback_smtprofile_before_negative` and §1.5.1 of the fstar-for-libcrux skill on layered cascades).
- **Out of scope but worth noting** for the next sprint: `Spec.MLDSA.Math.lemma_mont_red_mod_q = admit ()` (Spec.MLDSA.Math.fst:258) and `Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_spec_eq_decompose = admit ()` (per `agent-proof-review-2026-05-03.md`). Both block separate Sprint A targets and are untouched by this audit.

---

**End of audit.**

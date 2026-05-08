# Asymmetric Opaque Experiment Status

**Started:** 2026-05-07
**Goal:** Strip per-simd-unit Seq.index SMTPats and refactor `is_lane_range_poly` to bottom out at new `is_intb_array_opaque` per-simd-unit atom.
**Baseline cascade:** k!63 ~ 624,588 instances on query 60 of generate_key_pair.
**HEAD:** 9b5b75b4b

## Status: FAILED — DISCARDED

**Conclusion:** Hypothesis falsified. The deep multi-trigger SMTPat on
`lemma_is_lane_range_poly_lookup` was NOT the source of the k!63 cascade.

**Re-profile result:** k!63 still ~638,000 instances on query 60 (baseline
624,588). No drop. Cascade source lies elsewhere — neither
`is_lane_range_poly`'s shape nor the per-simd-unit SMTPats on
`is_pos_array_opaque` / `is_binary_array_8_opaque` are responsible.

**Collateral damage observed during the experiment:**
- 18+ queries throughout `generate_key_pair` regressed to "incomplete
  quantifiers" failures (queries 9, 19, 23, 24, 25, 29, 30, 36, 38, 40,
  41, 44, 49, ...) once the per-simd-unit `Seq.index x i` SMTPats were
  removed from `lemma_is_pos_array_lookup` and
  `lemma_is_binary_array_8_lookup`.
- `Encoding.Signature.deserialize` queries 45, 75, 94 also failed with
  similar reasons.
- Confirms that those SMTPats ARE load-bearing for many other functions
  (not just the cascade target). Auditing every call site to insert
  explicit `lemma_is_pos_array_lookup l x i;` would exceed the time
  budget by a wide margin.

**Reverted via** `git checkout HEAD -- libcrux-ml-dsa/src/` then
`./hax.sh extract`.  Tree clean at HEAD `9b5b75b4b`.

## Edits made (now reverted)
1. `src/simd/traits/specs.rs`: stripped `Seq.index` SMTPat from
   `lemma_is_pos_array_lookup` and `lemma_is_binary_array_8_lookup`;
   added `is_intb_array` / `is_intb_array_opaque` + intro/lookup lemmas
   (no SMTPat).
2. `src/polynomial.rs`: rewrote `is_lane_range_poly` body to bottom out
   at `is_intb_array_opaque`. Lookup lemma per-`j` only with
   opaque-atom multi-pattern trigger. Bridge lemmas updated.
3. `src/matrix.rs::compose_w_approx_per_row`: explicit
   Classical.forall_intro_2 bridge before `shift_left_then_reduce`.
4. `src/encoding/verification_key.rs::generate_serialized`: explicit
   per-iteration bridge before `t1::serialize`.

## Original baseline preserved
- HEAD: `9b5b75b4b ml-dsa: WIP keygen ntt-loop invariant + s1_s2 frame bridge`
- src/ tree clean.

Edits made:
1. `src/simd/traits/specs.rs`: stripped `Seq.index` SMTPat from `lemma_is_pos_array_lookup` and `lemma_is_binary_array_8_lookup`; added `is_intb_array` / `is_intb_array_opaque` + intro/lookup lemmas (no SMTPat).
2. `src/polynomial.rs`: rewrote `is_lane_range_poly` body to bottom out at `is_intb_array_opaque`. Lookup lemma now per-`j` only with opaque-atom multi-pattern trigger. Intro lemma's requires updated. Bridge lemmas (`lemma_lane_range_pos_to_bounded_poly`, `lemma_lane_range_pos_to_pos_array_slice`) updated to call the new per-`j` lookup + explicit `lemma_is_intb_array_lookup`.
3. `src/matrix.rs::compose_w_approx_per_row`: added explicit Classical.forall_intro_2 bridge before `shift_left_then_reduce` since the deep SMTPat is gone.
4. `src/encoding/verification_key.rs::generate_serialized`: added explicit bridge in loop body before `t1::serialize`.

Now extracting & rebuilding.

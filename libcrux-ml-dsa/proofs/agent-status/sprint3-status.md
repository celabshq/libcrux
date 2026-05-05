# Sprint 3 — matrix.rs body proofs

Started: 2026-05-04
Closed: 2026-05-05

## Outcomes

### Step 1 — `add_to_ring_element` wrapper (DONE)

Free function added in `src/matrix.rs` (not on `impl PolynomialRingElement`,
because that creates a `Polynomial → Polynomial.Spec → Polynomial`
extraction cycle: ml-dsa's `t_PolynomialRingElement` lives in `Polynomial`,
and `Polynomial.Spec.is_bounded_poly` references it; ml-kem doesn't hit this
because its `t_PolynomialRingElement` lives in `Vector`).

```rust
fn add_to_ring_element<S: Operations>(
    myself: &mut PolynomialRingElement<S>,
    rhs: &PolynomialRingElement<S>,
    _bound: usize,
)
// pre:  v _bound + FM <= MAX_I32 /\
//       is_bounded_poly _bound myself /\
//       is_bounded_poly FM rhs
// post: is_bounded_poly (_bound +! FM) myself_future
```

Body: calls `add_bounded` (per-simd-unit post), then a single
`lemma_is_bounded_poly_intro` to lift to poly-level.

### Step 2-4 — `compute_matrix_x_mask` (DONE)

Pre/post rewritten in `is_bounded_poly` form (function args have exact-length
slices: `Seq.length matrix == rows_in_a * columns_in_a`, etc., per the
caller's actual array shape). Body proof closes at z3rlimit 400 with
`--split_queries always`.

Key proof moves:
- Snapshot `let old_result = result.to_vec().as_slice()` before zeroing
  result[i]. Inner-loop frame: `forall k. k < v rows_in_a /\ k <> v i ==>
  result[k] == old_result[k]`.
- Inner inv: `is_bounded_poly (j *! FM) result[i]`.
- After `add_to_ring_element`: post is `is_bounded_poly (j *! FM +! FM)`;
  bridge to inner inv at j+1 (`is_bounded_poly ((j +! 1) *! FM)`) via
  `assert (...)` plus `Math.Lemmas.distributivity_add_left (v $j) 1 8380416`
  — opacity stops congruence-rewrite from firing automatically; the explicit
  distributivity lemma instance gives Z3 the missing nat-equality.
- After inner loop: weaken `cols * FM → 2_143_289_343` via
  `lemma_is_bounded_poly_higher`. Direct match for the new `reduce` pre.
- After `reduce` + `invert_ntt_montgomery`: weaken
  `4_211_177 → FM` via `lemma_is_bounded_poly_higher`.
- Outer-iter exit: re-establish outer inv at i+1 by spelling out the chain
  (frame from inner inv + Leibniz under outer-inv-at-entry's old_result
  bound) via four cascading `assert`s — Z3 can't synthesize the chain in one
  shot at z3rlimit 400.

### Step 3 (precondition cascade) — Ntt.fst upgraded to is_bounded_poly form

`src/ntt.rs` pre/post conditions for `ntt`, `invert_ntt_montgomery`,
`reduce`, `ntt_multiply_montgomery` rewritten from per-lane
`forall j<32. is_i32b_array_opaque b ...` to
`Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly b re`. Each body now
threads:
- `reveal_opaque (\`%is_bounded_poly) ...` at function entry to bridge into
  the per-lane form the underlying SIMDUnit::* trait method needs;
- a single `lemma_is_bounded_poly_intro` at body exit to lift the per-lane
  post back to poly-level.

This eliminates the `Classical.forall_intro weaken` per-lane bridges that
were scattered across consumers (`vector_times_ring_element`, the
`compute_matrix_x_mask` body, etc.) — those collapse to a single
`lemma_is_bounded_poly_higher` call at the bound transition.

Cascaded callers updated:
- `src/encoding/error.rs` — `deserialize_to_vector_then_ntt`: kept the
  existing per-lane `is_i32b_array_larger` weaken (lifts per-lane 11 →
  per-lane FM) and added a single `lemma_is_bounded_poly_intro` to lift to
  `is_bounded_poly FM` for `ntt`'s new pre.
- `src/encoding/t0.rs` — same recipe (per-lane `pow2 12 → FM`, then intro).
- `src/matrix.rs::vector_times_ring_element` — kept its existing per-lane
  pre/post (callers depend on it), added one `lemma_is_bounded_poly_intro`
  at function entry to lift `ring_element` to is_bounded_poly form, and
  replaced the old per-lane weaken with `lemma_is_bounded_poly_higher` +
  explicit `Classical.forall_intro lemma_lift` (calls
  `lemma_is_bounded_poly_lookup` per-lane to extract per-lane FM bound).

### Step 5 — `compute_as1_plus_s2` (BODY ADMIT REMAINS)

Sprint 2 blocker (hax extracts outer fold over `(a_as_ntt, result)` as a
tuple state, breaks F* field resolution) NOT addressed in this sprint.
Body remains `admit ()`; pre/post unchanged from per-lane form. The
prompt's suggested fix (refactor inner j-loop into a free fn taking
`&mut a_as_ntt[i*cols..(i+1)*cols]` and `&mut result[i]`) is straightforward
but not done — out of time budget.

### Step 6 — `compute_w_approx` (BODY ADMIT REMAINS)

Same nested-fold structure. Body remains `admit ()`; pre/post unchanged.

## Side effects on already-verified functions

`add_vectors` and `subtract_vectors` regressed at `--z3rlimit 800
--split_queries always` after the Polynomial.Spec / Ntt changes (Z3 was
exploring a larger SMTPat search space and timed out at 800 rlimit).
Dropping their per-fn options to `--z3rlimit 200 --ext context_pruning`
(default) restored them; their bodies are unchanged. **rlimit hard cap of
800 was contraindicated for these — lower rlimit + smaller search space
verifies faster.**

## Final state

```
make check/Libcrux_ml_dsa.Matrix.fst   →  exit 0, ~170s
make check/Libcrux_ml_dsa.Ntt.fst      →  exit 0
cargo test --release --lib             →  20 passed
```

Functions in `Matrix.fst` after Sprint 3:
- `add_to_ring_element`           — verified ✓ (new)
- `compute_as1_plus_s2`           — admitted (Sprint 2 blocker, see above)
- `compute_matrix_x_mask`         — **verified ✓** (Sprint 3 deliverable)
- `vector_times_ring_element`     — verified ✓ (post-Ntt-upgrade)
- `add_vectors`                   — verified ✓ (rlimit dropped to 200)
- `subtract_vectors`              — verified ✓ (rlimit dropped to 200)
- `compute_w_approx`              — admitted (Sprint 2 blocker)

## Lessons / non-obvious findings

1. **Opacity bridge for j*FM +! FM ↔ (j+1)*FM**: with `is_bounded_poly`
   `opaque_to_smt`, Z3 sees two numerically-equal but syntactically-different
   bound terms as distinct uninterpreted predicates. `assert (...)` alone
   doesn't close the gap. Needed `Math.Lemmas.distributivity_add_left (v j)
   1 8380416` to give Z3 the explicit nat equality, then congruence-rewrite
   under is_bounded_poly fires.

2. **Outer-iter exit subtyping check**: Z3 won't chain frame + Leibniz +
   final lemma_higher in one query at the loop boundary. Spell each
   intermediate fact out as an explicit `assert`.

3. **rlimit 800 + split_queries can be SLOWER than rlimit 200**: for
   per-lane forall-only proofs (add_vectors, subtract_vectors), the
   smaller search space at 200 rlimit closes faster than 800. The
   `--ext context_pruning` flag matters more than rlimit for these.

4. **`Polynomial.Spec` cycle**: when a spec module is a Rust submodule of a
   parent module that defines the type the spec quantifies over, you can't
   reference the spec from the parent module's body. Place wrappers in a
   sibling/consumer module. (ml-kem doesn't hit this because its
   `t_PolynomialRingElement` lives in `Vector`, separate from `Polynomial`.)

5. **The `v_FIELD_MAX` (u32) vs `mk_usize 8380416` (usize) mismatch**:
   `is_bounded_poly` takes `usize`, but the SIMD trait pre uses
   `v ${specs::v_FIELD_MAX}` (u32). Both have v=8380416 as ints, but Z3
   needs `v_mk_int_u32` and `v_mk_int_usize` to fire to equate. A
   `reveal_opaque (\`%is_bounded_poly) (is_bounded_poly (mk_usize 8380416)
   re)` at the body's start gives Z3 enough handle to bridge.

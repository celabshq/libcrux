# Plan: USER-15 + forward-NTT (inspired by the USER-14 closure) — 2026-05-31

Context: USER-14 (`invert_ntt_at_layer_4_plus` functional post) is **closed & verified**
(no `assume`, full `fstar_build` green; backport to `invert_ntt.rs` prepared, pending
extract+verify). Reusable assets it produced:
- In `Bridges.fst` (committed): `cross_vec_hyp`, `lemma_cross_vec_from_step`,
  `lemma_cross_vec_frame`, `lemma_layer_4_plus_cross_vector`, `lemma_layer_4_plus_per_coeff`,
  `lemma_ntt_inverse_layer_n_256_compose`, `lemma_ntt_inverse_layer_unfold`,
  `lemma_layer_4_plus_post_from_cross_vec`, `lemma_zeta_eq_vzetas` (axiom),
  `lemma_ntt_inverse_layer_n_16_{2,4,8}_lane`.
- In `Invert_ntt.fst` (verified scratch, commit 55cdf0a2d): the **store_block top-down recipe** —
  opaque named fold-invariants `outer_inv`/`inner_inv`, standalone maintenance lemmas
  (`lemma_inner_step_maintains`, `lemma_inner_to_outer`, `lemma_inner_inv_init`,
  `lemma_outer_inv_init`, `lemma_postloop_cross_vec`), per-index lookup lemmas
  (`lemma_outer_inv_lookup`/`lemma_inner_inv_lookup`), the opaque `inv_ntt_step_post`.

Two reusable lessons (apply to BOTH tasks below):
- **L1 (opaque named fold-invariants + standalone maintenance lemmas):** never do nested-fold
  maintenance inline; the function-level VC goes unstable. Factor each fold's invariant into an
  opaque `prop` predicate (one atom carried), and discharge maintenance in clean-context lemmas.
- **L2 (v-level arithmetic + lookup lemmas):** opaque predicates must use `v`-level (total int)
  arithmetic on indices/thresholds (free-param `*!`/`+!` ⇒ unprovable overflow VCs); instantiate
  a revealed opaque-pred forall at a specific `i` via a lookup lemma whose *ensures contains the
  trigger terms* (`coeffs.[i]`), never a bare assert.

---

## TASK A — USER-15: `invert_ntt_montgomery` (composition; ~2–4 h)

**Goal.** Discharge `to_spec_poly_mont re_future == ntt_inverse_butterflies (to_spec_poly_mont re)`
and remove the `admit () (* Panic freedom *)`. (`is_bounded_poly 3328 re_future` already flows from
the last layer's bounds post.)

**Pattern = COMPOSITION (not loop-threading).** `ntt_inverse_butterflies p = ntt_inverse_layer
(ntt_inverse_layer (… (ntt_inverse_layer p 1) …) 6) 7` (verified: `Hacspec_ml_kem.Invert_ntt.fst:167`).
The driver calls `layer_1; layer_2; layer_3; layer_4_plus(4); …(5); …(6); …(7)`, producing
`re_0 → re_1 → … → re_7`. Need each step in polynomial form
`to_spec_poly_mont re_i == ntt_inverse_layer (to_spec_poly_mont re_{i-1}) i`, then chain.

**Step-by-step:**
1. **Layers 4–7 steps are FREE** — `invert_ntt_at_layer_4_plus`'s post (USER-14) is *already*
   `to_spec_poly_mont re_i == ntt_inverse_layer (to_spec_poly_mont re_{i-1}) layer`. Just sequence them.
2. **Layers 1–3 bridge (the real work)** — their posts are per-16-vector
   (`mont_i16_to_spec_array 16 (f_repr re_i[k]) == ntt_inverse_layer_n 16 (… re_{i-1}[k] …) len_i (zetas…)`,
   k<16, len = 2/4/8). Write a Bridges lemma `lemma_intra_vec_layer_to_poly` (layer ∈ {1,2,3}, len = 2^layer < 16):
   from the 16 per-vector `ntt_inverse_layer_n 16 … len …` equalities derive
   `to_spec_poly_mont re_i == ntt_inverse_layer (to_spec_poly_mont re_{i-1}) layer`.
   Building blocks already in Bridges: `lemma_ntt_inverse_layer_n_16_{2,4,8}_lane` (intra-vector
   per-len), `lemma_ntt_inverse_layer_n_256_compose` + `lemma_ntt_inverse_layer_unfold` (the
   table→explicit + per-coeff→256 plumbing reused from USER-14). Key fact: for len<16 the butterfly
   stays *within* each 16-vector, so `ntt_inverse_layer p layer` = per-vector
   `ntt_inverse_layer_n 16 vec len` applied independently — the `_16_*_lane` lemmas give exactly that
   per vector; assemble across the 16 vectors with a `to_spec_poly_mont`-unfold (analogous to
   `lemma_layer_4_plus_per_coeff`). Apply the L2 zeta-table bridge (`lemma_zeta_eq_vzetas`) so the
   per-vector `zetas_{4,2,1}_ (zeta …)` match `ntt_inverse_layer`'s internal `v_ZETAS` table.
3. **Chain in `invert_ntt_montgomery`** — after each layer call, assert the polynomial-form step
   (steps 1–2), then a 7-fold `calc`/sequential-assert composes them to
   `ntt_inverse_butterflies (to_spec_poly_mont re_0)`. Mechanical once steps 1–2 hold. Mind the
   `is_bounded_poly_higher` coercions already in the body (they don't affect `to_spec_poly_mont`).
4. **Remove the panic-freedom `admit`**; backport to `invert_ntt.rs` (the layer calls already exist;
   add the per-step + chain `fstar!` asserts + the new Bridges lemma); re-extract; per-stage build.

**Reuses from USER-14:** `lemma_zeta_eq_vzetas`, `lemma_ntt_inverse_layer_unfold`,
`lemma_ntt_inverse_layer_n_256_compose`, `to_spec_poly_mont` unfold lemmas, the layer_4_plus posts.
**New:** `lemma_intra_vec_layer_to_poly` (Bridges) + ~7 chain asserts in the driver.
**Risk:** moderate — the layers-1–3 intra-vector→polynomial bridge is the only genuinely new lemma;
the rest is sequencing. NOT a Z3-wall task. Prior notes estimated "lane A (Bridges unfold) ~30–60 min
+ lane B (driver) ~30 min" — realistic given USER-14 unblocked it.

---

## TASK B — Forward NTT functional posts (THE "same pattern" reuse; larger)

**Goal.** Strengthen the forward NTT (`ntt.rs`) from bounds-only to functional, mirroring the inverse
chain: `ntt_at_layer_{1,2,3}` per-vector posts, `ntt_at_layer_4_plus` polynomial post (USER-14 recipe
**verbatim**), `ntt_at_layer_7`, and the forward driver → `ntt_binomial…`/`ntt` spec.

**Why it's the same pattern:** `ntt_at_layer_4_plus` (`ntt.rs:406`) has the IDENTICAL nested
`for round { for j { step; upd j; upd j+step } }` structure as `invert_ntt_at_layer_4_plus` — only
(a) the butterfly is forward Cooley–Tukey `(a + ζ·b, a − ζ·b)` instead of GS `(a+b, ζ·(b−a))`,
(b) `zeta_i` *increments*, (c) bound is `_initial_coefficient_bound (+3328)`. So the entire
store_block top-down scaffold ports directly.

**Step-by-step (forward layer_4_plus):**
1. **Forward Bridges analogs** in `Bridges.fst`: `cross_vec_hyp_fwd` (forward butterfly relation),
   `lemma_cross_vec_from_step_fwd`, `lemma_cross_vec_frame` (reuse as-is — frame is direction-agnostic),
   `lemma_ntt_layer_n_256_compose_fwd` + `lemma_ntt_layer_unfold_fwd` (forward `ntt_layer`/`ntt_layer_n`
   spec), `lemma_layer_4_plus_post_from_cross_vec_fwd`. Plus a forward per-step bridge
   `ntt_step_post`/`lemma_inv...→fwd` (the `ntt_layer_int_vec_step` analog of
   `inv_ntt_layer_int_vec_step_reduce`). The forward `step` fn likely needs the same opacity fix
   (opaque `ntt_step_post`).
2. **Port the recipe verbatim** to `ntt_at_layer_4_plus`'s `.fst`: opaque `outer_inv_fwd`/`inner_inv_fwd`
   (same shape, forward threshold/zeta), the 5 maintenance/init/bridge lemmas + 2 lookups + keystone +
   numeric facts (`lemma_offset_vec`/`lemma_inner_index`/`lemma_layer_numeric_facts` are
   direction-agnostic — copy as-is). Apply L1 + L2 throughout.
3. **Forward layers 1–3** functional posts (mirror inverse layers 1–3 — those are already proven, so
   their structure is a template).
4. **Forward driver** composition (mirror TASK A) → forward `ntt` spec.
5. Backport each to `ntt.rs`; re-extract; per-stage build.

**Reuses verbatim:** `lemma_offset_vec`, `lemma_inner_index`, `lemma_layer_numeric_facts` (zeta_i
init differs — adjust the `e_zeta_i_init` values), `lemma_cross_vec_frame`, the entire
`outer_inv`/`inner_inv` + maintenance/lookup *skeleton* (rename + swap butterfly).
**New:** forward butterfly relation + forward composition lemmas (the `_fwd` family).
**Risk:** medium-high effort but LOW novelty — it's a mechanical port of a now-proven recipe. Best
done after USER-15 (so the inverse side is fully closed as the reference template). Likely
splits into forward-USER-12/13 (layers 1–3), forward-USER-14 (layer_4_plus), forward-USER-15 (driver).

---

## Recommended order & dependencies
1. **Finish USER-14 backport** (extract + verify `invert_ntt.rs`) — gates a clean inverse side.
2. **USER-15** (`invert_ntt_montgomery`) — small, unblocked, completes the inverse NTT functional chain.
3. **Forward NTT** (Task B) — the big same-pattern reuse; start with `ntt_at_layer_4_plus` (verbatim
   recipe port) since it's the highest-confidence reuse, then forward layers 1–3, then the forward driver.

All three are read-only-plannable now; none needs the parallel sha3 build's RAM until the
extract/verify steps.

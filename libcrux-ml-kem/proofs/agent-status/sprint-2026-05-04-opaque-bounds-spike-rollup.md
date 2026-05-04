# Sprint 2026-05-04 — opaque-bounds spike (PARTIAL — primary deliverable landed)

**Branch:** `libcrux-ml-kem-opaque-bounds-spike` (worktree `../libcrux-opaque-spike`)
**Tip on entry:** `080c17468`  (cascade infra partial)
**Commits added (3):**

  1. `501888222` — opacity refactor: `[@@ "opaque_to_smt"]` on
     `is_bounded_polynomial_{vector,matrix}` + intro/elim lemmas.
  2. `e01ad3b73` — toolkit: nat-indexed `elim_nat` SMTPat'd lemmas +
     `lemma_is_bounded_polynomial_vector_higher` widening lemma.
  3. `d2b34d7dc` — flip `encrypt_c1` + thread `encrypt_unpacked` requires.

**Goal:** validate the hypothesis that the three SMT walls blocking
`encrypt_c1` lax→panic_free (sprint 2026-05-07a, partial-rollup §"Wall 1/2/3")
were caused by the nested-forall body of `is_bounded_polynomial_matrix`
flooding Z3's instantiation budget on unrelated queries.

**Outcome:** **hypothesis confirmed.** Walls 1 and 2 fall *automatically*
with opacity (no body intervention).  Wall 3 requires two explicit
helpers (widening + intro) but is now tractable at rlimit 400 with
`--split_queries always`.  `encrypt_c1` flipped lax → panic_free.
`encrypt_c2` remains lax pending one independent producer-side strengthening.

## What landed

### Opacity (commit 501888222)

`src/polynomial.rs`:
* `is_bounded_polynomial_vector`  → `[@@ "opaque_to_smt"]`
* `is_bounded_polynomial_matrix`  → `[@@ "opaque_to_smt"]`
* `lemma_is_bounded_polynomial_{vector,matrix}_intro`  (Rust-side, no SMTPat)
* `lemma_is_bounded_polynomial_{vector,matrix}_elim`  (raw F* via
  `fstar::after`, `usize`-indexed, dual-trigger multi-pattern
  `[SMTPat (Seq.index arr (v i)); SMTPat (is_bounded_... b arr)]`)

Strategy: same idiom as `Vector.Traits.Spec.is_i16b_array_opaque` +
`lemma_bounded_i16_array_lookup` (the pre-existing leaf opaque atom).

**Surprising finding (option (a) over (b) over (c) for the user's
mid-spike fork):** producer sites (generate_keypair_unpacked,
build_unpacked_public_key_mut, deserialize_ring_elements_reduced,
compute_As_plus_e wrapper consumers) **verify clean with no manual
`lemma_intro` calls.**  Z3's E-matcher fires the SMTPat'd ELIM in
both directions (consume + produce) given the multi-pattern.

### Toolkit (commit e01ad3b73)

`src/polynomial.rs` adds, again in `fstar::after`:
* `lemma_is_bounded_polynomial_{vector,matrix}_elim_nat` — `nat`-indexed
  companion to the usize forms.  Needed because consumer aux lemmas
  inside `Classical.forall_intro` typically use `i: nat`, and the
  trigger `Seq.index arr i_nat` fails to unify with `Seq.index arr (v i_usize)`.
  The nat form delegates by `assert (v (mk_int #usize_inttype i) == i)`.
* `lemma_is_bounded_polynomial_vector_higher` — widen `b1 → b2` on the
  opaque vector atom.  Discharged via `Classical.forall_intro` over a
  per-`i` `is_bounded_poly_higher` call, re-folded by intro.  rlimit 400
  with `--split_queries always`.

### Site flips (commit d2b34d7dc)

`src/ind_cpa.rs`:
* `encrypt_c1`: lax → panic_free.  Requires:
  `is_bounded_polynomial_matrix(3328, matrix)`.  Ensures:
  `is_bounded_polynomial_vector(3328, r_as_ntt) /\ is_bounded_poly(3328, error_2)`.
  Body adds two `hax_lib::fstar!()` helpers:
    1. `lemma_is_bounded_polynomial_vector_higher $K $error_1 (sz 3) (sz 7)`
       — widen sampler bound to compute_vector_u's needed bound.
    2. `lemma_is_bounded_polynomial_vector_intro $K $u (sz 3328)` —
       fold compute_vector_u's per-element forall ensures into the
       opaque atom required by compress_then_serialize_u.
* `encrypt_unpacked`: thread `is_bounded_polynomial_matrix(3328, public_key.A)`
  requires (cascade up one level — its body's encrypt_c1 call now
  requires the bound).
* `encrypt_c2`: stays lax with FOLLOW-UP comment (see "What did NOT land").

### Wall fates

| Wall | Prior diagnosis | Spike outcome |
|---|---|---|
| 1 — `sample_ring_element_cbd` `K+K<256` | Needed `assert (v K <= 4)` | **Fell automatically with opacity.** No hint added. |
| 2 — `Seq.equal $prf_input` flood | Bounds in requires inflated context | **Fell automatically with opacity.** No hint added. |
| 3 — `compute_vector_u` per-element pre | Needed `is_bounded_poly_higher` per-element | Required `lemma_is_bounded_polynomial_vector_higher` (3→7) + `lemma_is_bounded_polynomial_vector_intro` (fold u). |

## What did NOT land — and why

### encrypt_c2 (1 site of 2)

`encrypt_c2` body's `compute_ring_element_v` call requires:

  * `is_bounded_polynomial_vector(3328, t_as_ntt)`  — supplied via new requires (would work)
  * `is_bounded_polynomial_vector(3328, r_as_ntt)`  — supplied via new requires (would work)
  * `is_bounded_poly(3328, error_2)`  — supplied via new requires (would work)
  * **`is_bounded_poly(3328, message)`  — NOT exported by
    `deserialize_then_decompress_message`'s current ensures.**

`Libcrux_ml_kem.Serialize.deserialize_then_decompress_message`'s
ensures only states the spec equivalence
`poly_to_spec result == hacspec deserialize_then_decompress_message ...`
— no pointwise bound.  In reality the values are 0 or 1664 (decompress_1
of binary input), so `is_bounded_poly 3328` is sound — but it has to
be *declared*.

This is a **producer-side strengthening on a different module** that
falls outside the spike's spec-only scope.  Adding it now would
re-typecheck Serialize.fst (~30s wall) and then `encrypt_c2` should
pass with the same body shape as `encrypt_c1` (a `Classical.forall_intro`
of `lemma_is_bounded_polynomial_vector_elim_nat` for both `t_as_ntt`
and `r_as_ntt`, no further widening needed — bound is already 3328).
Estimated 30 min for the producer change + encrypt_c2 flip in a clean
follow-up session.

### Cascade up (encapsulate, decapsulate, instantiations, mlkem*::encapsulate)

Not started.  Adding `is_bounded_polynomial_matrix(3328, public_key.A)` to
`encrypt_unpacked`'s requires breaks `Ind_cca.Unpacked::encapsulate`
which calls into `encrypt_unpacked` — but **that module's `.fst` body
was already non-verifying pre-spike** (per sprint 2026-05-07a rollup
§"Hint-replay risk": "no baseline `.checked` cache (only `.fsti.checked`)").
The `.fsti` re-verifies clean.  So the cascade pause is on a module
that wasn't verifying anyway.  Independent re-baseline work needed
to resume the cascade.

### Site 3 (deserialize_then_decompress_u)

Out of scope — independent thread (no opacity dependency).  Tracked
in the prior sprint's FOLLOW-UP comment in src/serialize.rs.

## Acceptance check

```
$ grep -c "verification_status(lax)" src/ind_cpa.rs
2     # encrypt_c2 + deserialize_then_decompress_u (out-of-scope thread)
$ grep -c "verification_status(lax)" src/ind_cca.rs
0

$ cd proofs/fstar/extraction
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Polynomial.Spec.fst
rc=0   (~2.4s)
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Matrix.fst
rc=0
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Serialize.fst
rc=0
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cpa.fsti
rc=0
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cpa.fst
rc=0   # encrypt_c1, encrypt_unpacked panic_free; encrypt_c2 admit() body still lax-skipped
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cpa.Unpacked.fst
rc=0
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cca.Unpacked.fsti
rc=0
```

Goal "3/3 lax sites flipped" — NOT achieved (1/3 flipped: `encrypt_c1`).
Spike's primary deliverable — opacity infrastructure + toolkit + one
real-world site flip validating the hypothesis — landed cleanly.

## Memory inputs to update

* `feedback_smtpat_percent_above_trait` — confirmed: opaque atom +
  SMTPat'd dual-trigger elim is the right pattern for layered bound
  predicates.  `(Seq.index arr i, opaque_atom)` multi-pattern works.
* New observation: SMTPats with **both** the indexed access pattern
  and the opaque-atom pattern fire bidirectionally — no separate
  `lemma_intro` calls needed at producer sites in practice.  This is
  the surprising "free producer" finding from option (a) over (b).
* `feedback_proof_debug_budget` (30-60 min cap): respected.  encrypt_c2
  attempt deferred at ~10-min C-tactic cap rather than chasing the
  producer-side strengthening.

## Files touched

* `src/polynomial.rs`: opacity decorations + 4 new lemmas (intro×2,
  elim×2 [usize+nat each], higher×1).
* `src/ind_cpa.rs`: encrypt_c1 lax→panic_free with new requires/ensures
  + 2 body helpers; encrypt_unpacked threaded; encrypt_c2 left lax with
  precise FOLLOW-UP comment.

## Next-session unblocks

1. **Strengthen `deserialize_then_decompress_message`'s ensures** with
   `is_bounded_poly(3328, result)`.  Single-line ensures change in
   `src/serialize.rs`; SMT discharge from existing decompress_1 bound
   (values are 0 or 1664).  Then re-apply encrypt_c2 lax→panic_free
   with the body shape from this spike's reverted attempt
   (Classical.forall_intro × 2 of elim_nat for t_as_ntt and r_as_ntt;
   git diff at d2b34d7dc^^ has the body sketch).

2. **Cascade up to encapsulate/decapsulate**: thread
   `is_bounded_polynomial_matrix(3328, ind_cpa_public_key.A)` requires
   from `encrypt_unpacked` up through `Ind_cca::encapsulate`,
   `Ind_cca::decapsulate`, `instantiations.rs::{encapsulate,decapsulate}`
   (3 backends × 2 ops), `mlkem{512,768,1024}::encapsulate`.  Estimated
   1-1.5h once Ind_cca.Unpacked.fst body's pre-existing failure
   (line 151, `serialize_public_key_mut` requires) is independently
   unblocked.

3. **Independent baseline:** re-establish Ind_cca.Unpacked.fst body
   verification (pre-existing broken, blocking step 2's cascade arrival).

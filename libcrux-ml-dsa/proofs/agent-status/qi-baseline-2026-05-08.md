# SMT QI baseline — 2026-05-08

Reference profile of dominant quantifier-instantiation patterns across
the top-cost ML-DSA queries, captured against HEAD `9b5b75b4b` plus the
tactical `generate_key_pair` body-admit (this session).  Companion to
`fstar-perf-top20.md`'s 2026-05-08 snapshot.  Will be re-run after the
trait-opacity remediation (audit Phase A) lands so we can measure
delta.

Method: ran `OTHERFLAGS='--log_queries --z3refresh' make -k` on the
target modules to dump `queries-<Module>-N.smt2` per query, then
`timeout 120 z3-4.13.3 smt.qi.profile=true smt.qi.profile_freq=20000
<file>` on the slowest query per target function, parsing
`[quantifier_instances]` lines.

For each profile, the table below shows:
- "max instances": the highest single-snapshot count (qi.profile prints
  periodic snapshots at the configured frequency)
- "name": as-printed by Z3.  Anonymous Skolems are `k!N`; named
  quantifiers begin with the F\* axiom name or a hash-suffixed term.

## Profile 1 — `Simd.Avx2::impl_1` query 1 (cold, FAILED-then-OK at rlimit 80, 12.1s)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Avx2-6.smt2` (111 MB)
Z3 ran for 8 s before timeout (incomplete result).

| max instances | name | notes |
|---:|---|---|
| 100,937 | `k!61` | **dominant** — anonymous Skolem from inner forall in some refinement |
| 29,418 | `Rust_primitives.Integers_pretyping_1eff91dc290b8194aeb15d2394025944` | int-typing axiom |
| 25,449 | `refinement_interpretation_Tm_refine_542f9d4f129664613f2483a6c88bc7c2` | `Prims.nat` refinement |
| 23,661 | `fuel_guarded_inversion_Rust_primitives.Integers.int_t` | fuel inversion |
| 21,362 | `Prims_pretyping_ae567c2fb75be05905677af440075565` | int pretyping |
| 20,325 | `Prims_pretyping_f537159ed795b314b4e58c260361ae86` | second int pretyping |
| 20,325 | `bool_inversion` | bool inversion |

## Profile 2 — `Simd.Avx2::impl_1` query 522 (succeeded with hint, 6.6s, used rlimit 41)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Avx2-527.smt2` (2.8 MB)

| max instances | name | notes |
|---:|---|---|
| 85,495 | `Prims_pretyping_ae567c2fb75be05905677af440075565` | int pretyping |
| 70,433 | `k!61` | anonymous, dominant |
| 44,911 | `refinement_interpretation_Tm_refine_a561f4db9d08b24320d2e11f5d6359be.1` | refinement on `Libcrux_ml_dsa.Simd.Avx2.Vector_type.to_coefficient_array`'s output array (line 36); fires once per to_coefficient_array call in body |
| 43,403 | `projection_inverse_BoxInt_proj_0` | BoxInt projection |
| 41,439 | `constructor_distinct_BoxInt` | BoxInt constructor distinctness |
| 41,119 | `int_typing` | int typing axiom |

## Profile 3 — `Simd.Avx2.Arithmetic::power2round` query 1 (succeeded, 3.3s, used rlimit 16)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Avx2.Arithmetic-26.smt2` (1.2 MB)
Note: q1 also failed once at rlimit 80 (incomplete quantifiers, with hint), then succeeded — borderline.

| max instances | name | notes |
|---:|---|---|
| 39,271 | `Prims_pretyping_ae567c2fb75be05905677af440075565` | int pretyping |
| 23,133 | `k!61` | anonymous |
| 20,344 | `projection_inverse_BoxInt_proj_0` | BoxInt projection |
| 18,844 | `int_typing` | int typing |
| 18,789 | `constructor_distinct_BoxInt` | BoxInt constructor |

## Profile 4 — `Simd.Portable.Ntt::ntt_at_layer_3_` query 647 (succeeded with hint, 690 ms)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Portable.Ntt-2104.smt2` (5.1 MB)

| max instances | name | notes |
|---:|---|---|
| 436 | `k!61` | low — hint replay is doing the work; cold cost was 71s with 36 queries |
| 392 | `int_inversion` | int inversion |
| 232 | `primitive_Prims.op_Addition` | + axiom |
| 214 | `projection_inverse_BoxInt_proj_0` | BoxInt projection |
| 191 | `int_typing` | int typing |

**Note**: this query's qi.profile is dominated by hint-replay shortcuts.
The real cold-cache cliff for this function is in the OTHER queries
that ran (684 → 36 query collapse from 2026-04-29b → 2026-05-08).
The cold-cache cliff profile will surface in the hint-deletion experiment.

## Profile 5 — `Simd.Portable.Invntt::invert_ntt_at_layer_3_` query 647 (succeeded with hint, 957 ms)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Portable.Invntt-2077.smt2` (5.0 MB)

| max instances | name | notes |
|---:|---|---|
| 364 | `k!61` | low — same as Ntt counterpart, hint-replay |
| 264 | `primitive_Prims.op_Addition` | + axiom |
| 248 | `int_inversion` | int inversion |
| 164 | `proj_equation_Rust_primitives.Integers.MkInt_@_0` | mk_int projection |
| 164 | `equation_Rust_primitives.Integers.v` | `v` unfold |
| 146 | `equation_Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial` | **module-internal helper** — `is_i32b_polynomial` is non-opaque; unfolds 146× per query |

**Observation**: `is_i32b_polynomial` is a Portable-internal helper
(NOT the opaque `is_bounded_poly` from the audit).  Likely candidate
for opacification in a follow-up if it's pervasive across the Invntt
module.  Does NOT affect the trait surface.

## Profile 6 — `Simd.Portable.Encoding.T0::deserialize` query 142 (succeeded with hint, 354 ms)

`.smt2`: `queries-Libcrux_ml_dsa.Simd.Portable.Encoding.T0-162.smt2` (4.2 MB)
Note: q84 of the same function FAILED at rlimit 0.05 (with hint, incomplete quantifiers) earlier.

| max instances | name | notes |
|---:|---|---|
| 4,959 | `Prims_pretyping_ae567c2fb75be05905677af440075565` | int pretyping |
| 3,432 | `k!61` | anonymous |
| 2,562 | `projection_inverse_BoxInt_proj_0` | BoxInt projection |
| 2,271 | `int_typing` | int typing |
| 2,271 | `constructor_distinct_BoxInt` | BoxInt constructor |
| 627 | `int_inversion` | int inversion |
| 511 | `bool_inversion` | bool inversion |

`Simd.Portable.Encoding.T0::deserialize` was new on this snapshot
(7.7 s in cold-cache; not present in 2026-04-29b at this magnitude).
Likely tied to F-7/F-8/F-10 t0 strict-lower-bound work; medium-priority
qi.profile target after trait-opacity remediation.

## Cross-cut summary

**Universal pattern across all six profiles**: anonymous `k!61` Skolem
dominates or co-dominates with int-typing axioms (`Prims_pretyping`,
`int_typing`, `BoxInt` constructor/projection, `int_inversion`).  This is
characteristic of **forall-over-int proofs that drive Z3 through the int
constructor/projection machinery** at every instantiation.

The k!61 magnitude correlates with cliff severity:
- 100K (Avx2.impl_1 q1, cold-cliff failure) — k!61 is the cascade
- 70K (Avx2.impl_1 q522, with-hint succeed) — k!61 still the cascade, just contained
- 23K (power2round q1, borderline) — moderate
- 3K (T0.deserialize q142, fast) — manageable
- ~400 (Ntt/Invntt at_layer_3_ q647, hint-replay) — replay shortcuts most work

**Important caveat**: `k!N` numbering is per-`.smt2` file local — `k!61`
in different queries may correspond to different anonymous Skolems
structurally.  What's consistent is the *pattern* (a single anonymous
Skolem dominating int-typing-heavy queries), not the identity.

**For the trait-opacity remediation**: the dominant non-anonymous
quantifiers are all int-typing infrastructure that we can't easily
make opaque.  The leverage is on collapsing the *number of forall
instantiations* by replacing bare per-element foralls in trait
pre/post (audit Phase A items) with opaque-pred atoms.  Each opaque
atom counts as 1 quantifier instance, not N.

**Specific candidate for separate follow-up** (not in the trait audit):
`Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial` is a non-opaque
Portable-internal helper that fires 146× in Invntt.invert_ntt_at_layer_3_
q647 (and likely more in colder queries).  Wrapping with
`[@@ "opaque_to_smt"]` is a low-risk single-module change.

## Next experiment: hint deletion

To measure cold-cache (no-hint) cliffs that are currently masked by
hint replay, the next step is to back up `.fstar-cache/hints/` and
re-run `JOBS=4 ./hax.sh prove`.  Functions that "FAILED-then-OK"
in the cold-cache run today survived because F\* recorded fresh
hints during the rebuild; without hints, those FAILED-then-OK
queries may fail outright at rlimit ceiling.

Per the user, surfacing those failures (by surgical per-function
admit-and-continue) is part of the comprehensive baseline so we
have:
- a list of "proofs that need hints to land in <2 min"
- a list of "proofs that fail without hints" (i.e. structurally
  rlimit-saturated even cold)
- a list of "proofs that succeed without hints"

The trait-opacity remediation should aim to move items from the first
two categories into the third.

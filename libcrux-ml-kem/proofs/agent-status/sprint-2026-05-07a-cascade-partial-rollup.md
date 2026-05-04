# Sprint 2026-05-07a — bounds cascade infrastructure (PARTIAL)

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `78e6b9092`
**Goal:** Eliminate the final 3 `verification_status(lax)` annotations in `ind_cpa.rs`.
**Outcome:** 3/3 lax sites unchanged. Producer-side ensures cascade landed; consumer-side flips reverted after hitting hard SMT walls.

## What landed

| File:Fn | Change | Why |
|---|---|---|
| `serialize.rs::deserialize_ring_elements_reduced` | +`is_bounded_polynomial_vector(3328, future)` ensures conjunct | needed for encrypt_c2's t_as_ntt bound (panic_free → admitted, forward-compat) |
| `ind_cpa.rs::generate_keypair_unpacked` | +`is_bounded_polynomial_matrix(3328, future(public_key).A)` ensures conjunct | needed for encrypt_c1's matrix bound |
| `ind_cpa.rs::build_unpacked_public_key_mut` | +both bound conjuncts (vector + matrix) on ensures | OTHER pk producer (used by ind_cpa::encrypt) |
| `serialize.rs::deserialize_then_decompress_ring_element_u` | +`is_bounded_poly(3328, result)` ensures conjunct | needed by site 3's loop invariant |
| `ind_cpa.rs::encrypt_c1` FOLLOW-UP comment | rewrote with body-blocker diagnosis | next-session leverage |
| `ind_cpa.rs::encrypt_c2` FOLLOW-UP comment | rewrote with bridge to encrypt_c1 strengthened ensures | next-session leverage |

All touched modules (Ind_cpa.fst, Ind_cca.fst, Serialize.fst) verify clean.

## What did NOT land — and why

**Site 1 (encrypt_c1) and Site 2 (encrypt_c2) lax → panic_free flips.**

Attempted full cascade: added bound requires to encrypt_c1, encrypt_c2,
encrypt_unpacked; bumped fsti rlimit via `before(interface, "#push-options
\"--z3rlimit 400\"")` (verified injection works). The fsti
typecheck PASSED with this rlimit bump.

Body verification at `--z3rlimit 400 --ext context_pruning --split_queries
always` failed at three queries, each consuming full rlimit (~70s wall):

| Line | What | Diagnosis |
|---|---|---|
| `Ind_cpa.fst:734` | `sample_ring_element_cbd` requires discharge (`domain_separator + K < 256`) | Z3 has `domain_separator == K` from existing fstar! hint, but doesn't easily derive `K + K < 256` from `is_rank K`. Likely needs explicit `assert (v K <= 4)`. |
| `Ind_cpa.fst:751` | Existing `Seq.equal $prf_input (Seq.append $randomness (Seq.create 1 $domain_separator))` assert | Was tractable at low rlimit pre-bound-additions; new bound predicates in requires inflate Z3 context, killing the seq query. Likely needs `Seq.lemma_eq_intro` style hints or splitting. |
| `Ind_cpa.fst:767` | `compute_vector_u` requires discharge (`is_bounded_poly(7, error_1[i])`) | sample_ring_element_cbd ensures `is_bounded_poly(3, ...)`. Need explicit `is_bounded_poly_higher` widening 3→7 per-element before the call. |

Each iteration costs ~3-5min wall. Reverted to lax to land a clean partial.

**Site 3 (deserialize_then_decompress_u).** Not attempted. Producer-side
ensures conjunct on `deserialize_then_decompress_ring_element_u` IS in place;
loop-invariant maintenance + `poly_to_spec_eq_to_spec_poly_plain` bridge work
remains for a dedicated 60-90 min session.

## Cascade map (partial — what's plumbed)

```
sample_matrix_A ✓ (existing)            compute_As_plus_e ✓ (existing)
        │ matrix bound 3328                       │ vector bound 3328
        ▼                                          ▼
generate_keypair_unpacked  +matrix bound (added 2026-05-07a) ✓
build_unpacked_public_key_mut  +both bounds (added 2026-05-07a) ✓
deserialize_ring_elements_reduced  +vector bound (added 2026-05-07a) ✓
        │
        ▼
encrypt_unpacked: NOT YET REQUIRES bounds  ← cascade pause point
        │
        ▼
encrypt_c1, encrypt_c2: STILL LAX
deserialize_then_decompress_u: STILL LAX (independent thread)
```

## Next-session steps (tight)

1. **Continue cascade up:** add `is_bounded_polynomial_{matrix,vector}(3328,
   public_key.{A,t_as_ntt})` requires to `encrypt_unpacked`. The fsti rlimit
   bump pattern (`before(interface, ...)`) is verified working. Cascade up to
   `ind_cca::encapsulate`/`decapsulate` (panic_free) which take
   `MlKemPublicKeyUnpacked` — must add same requires. Then to
   `ind_cca::instantiations` (3 backends) and `mlkem{512,768,1024}::encapsulate`
   (unpacked variants). The `mlkem768::encapsulate` is unannotated Rust —
   either add hax requires there too, or add an `assume` block at that
   boundary marked FOLLOW-UP.

2. **Then site 1+2 body discharge:** with cascade plumbed, encrypt_c1 body
   needs the 3 explicit hints documented above. Estimate 30-60 min after
   cascade lands.

3. **Site 3 independently:** add `poly_to_spec_eq_to_spec_poly_plain` bridge
   call inside the loop body (per existing FOLLOW-UP comment). 60-90 min.

## Hint-replay risk

`Ind_cca.Unpacked.fst` had no baseline `.checked` cache (only `.fsti.checked`)
— the `.fst` rebuild fails at line 404 (`serialize_unpacked_secret_key` call,
needs is_bounded_polynomial_vector for t_as_ntt, secret_as_ntt) at rlimit 80
hint replay. This pre-dates this sprint (verified by `git stash` baseline run).
Out of scope for this rollup but tracked.

## Files touched

- `src/ind_cpa.rs`: ensures additions (generate_keypair_unpacked,
  build_unpacked_public_key_mut), FOLLOW-UP comments rewritten on encrypt_c1,
  encrypt_c2.
- `src/serialize.rs`: ensures additions (deserialize_ring_elements_reduced,
  deserialize_then_decompress_ring_element_u).

## Acceptance check

```
$ grep -c "verification_status(lax)" src/ind_cpa.rs src/ind_cca.rs
src/ind_cpa.rs:3
src/ind_cca.rs:0

$ make check/Libcrux_ml_kem.{Ind_cpa,Ind_cca,Serialize}.fst
rc=0
```

3/3 lax remain in `ind_cpa.rs` per goal — NOT achieved. Cascade infrastructure
landed cleanly.

# Phase F Stream 2 — Pattern-1 eq_intro cluster (ind_cpa)

Date: 2026-05-03
Worktree: /Users/karthik/libcrux-stream2-ind_cpa
Branch: agent-mlkem/phase-f-stream2-ind_cpa
Base: b9eee5838 (libcrux-ml-kem-proofs)

## Per-fn flip table

| Fn | Family | Status | Notes |
|-----|--------|--------|-------|
| `deserialize_vector` | B (no NTT)  | **FLIPPED** lax → panic_free | Q1 105ms @ 0.063 rlimit, Q2 202ms @ 1.273 rlimit (cap 800) |
| `deserialize_then_decompress_u` | B + NTT | stays lax (deeper blocker) | Lemma_post discharged, but loop-inv maintenance fails: `ntt_vector_u` ensures is `is_bounded_poly` only — its functional ensure `poly_to_spec(future(re)) == ntt(poly_to_spec(re))` is commented out. Out of stream scope. |
| `serialize_vector` | A | stays lax (not attempted) | Per-fn budget exhausted on Family B NTT diagnosis. Existing `eq_intro out (serialize_secret_key K T_SIZE v)` requires fold-unrolling lemma in spec module (`serialize_secret_key` is a `Hax.Folds.fold_range` with no per-chunk postcondition); spec-module change out of "develop locally" scope without a draft lemma in consumer file. |
| `compress_then_serialize_u` | A | stays lax (not attempted) | Same fold-unrolling structural blocker as `serialize_vector`. |

## Common proof-pattern that emerged (lighthouse `deserialize_vector`)

The post-loop `eq_intro` between two `Hacspec_ml_kem.Parameters.createi`-built
arrays succeeds at panic_free rlimit 800 ONLY if Z3 has explicit help to
match per-index forms.  The pattern that worked:

```fstar
let lemma_post (j: nat) : Lemma
    (requires j < v $K)
    (ensures
      Seq.index (vector_to_spec K secret_as_ntt) j ==
      Seq.index (vector_decode_12_ K secret_key) j) =
  let slice = Seq.slice secret_key (j * BPRE) (j * BPRE + BPRE) in
  let chunk : t_Array u8 (mk_usize 384) =
    Core_models.Result.impl__unwrap (Core_models.Convert.f_try_into slice) in
  eq_intro (chunk <: Seq.seq u8) slice
in Classical.forall_intro (Classical.move_requires lemma_post);
eq_intro (vector_to_spec K secret_as_ntt) (vector_decode_12_ K secret_key)
```

Three key ingredients:
1.  **Per-index Lemma (with `requires`/`ensures`)** to scope the goal
    to a single index `j < K` — keeps the body's quantifier load small.
2.  **`Classical.forall_intro` + `Classical.move_requires`** to lift the
    Lemma to a universal fact at the `eq_intro` site.
3.  **Inner `eq_intro`** between `try_into-unwrapped t_Array` (cast as
    Seq u8) and the original `Seq.slice secret_key ...` — bridges
    `array_from_fn`-built `t_Array` with the loop-invariant's
    `Seq.slice` form so `byte_decode` calls match syntactically.

Then the outer `eq_intro` reduces to per-index `Seq.index` equality
which the lemma + `createi_lemma` SMTPat (in `Hacspec_ml_kem.Parameters`)
discharges via single trigger.

## Why Family B + NTT is genuinely harder

The same pattern was applied to `deserialize_then_decompress_u`.  The
post-loop `lemma_post` was structurally cleaner (no try_into bridge —
the spec slices the same buffer) and did discharge.  But Z3 timed out
on **query 2** (the body itself) at rlimit 800 in 142 s with reason
`canceled`.  Root cause:

* loop body: `u_as_ntt[i] = decompress_decode(...); ntt_vector_u(&mut u_as_ntt[i]);`
* loop invariant: `poly_to_spec(u_as_ntt[j]) == ntt(decompress(byte_decode_dyn(slice)))` for j ≤ i.

Establishing this invariant after `ntt_vector_u(&mut u_as_ntt[i])`
requires `poly_to_spec(u_as_ntt[i]_future) == ntt(poly_to_spec(u_as_ntt[i]))`.
That ensure is **commented out** at `src/ntt.rs` lines 358–359:

```rust
// #[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector ${re}_future ==
//     Hacspec_ml_kem.Ntt.ntt (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#))]
pub(crate) fn ntt_vector_u<...>(re: &mut PolynomialRingElement<Vector>) { ... }
```

`ntt_vector_u`'s only live ensures is `spec::is_bounded_poly(3328, future(re))`.

→ `deserialize_then_decompress_u` is **blocked at the loop-invariant
maintenance step**, not at the eq_intro post.  Reverted source to lax
with FOLLOW-UP comment naming the exact missing ensures.

## Why Family A is structurally blocked

`serialize_vector` and `compress_then_serialize_u` post-loop `eq_intro`s
target `Hacspec_ml_kem.Serialize.serialize_secret_key K T v` and
`compress_then_serialize_u K U v du` respectively.  Both spec functions
have body `Rust_primitives.Hax.Folds.fold_range` with **`(fun _ -> Prims.l_True)`**
postcondition.  Without a per-chunk post or an explicit unrolling
lemma, Z3 cannot reduce
`Seq.slice (serialize_secret_key K T v) (j*B) ((j+1)*B) ==
byte_encode 384 3072 v[j] 12`.  The lighthouse pattern used the
`createi_lemma` SMTPat (defined in `Hacspec_ml_kem.Parameters`); no
analogue exists for `Hax.Folds.fold_range`.  A consumer-side ad-hoc
unrolling lemma would be ≥ K rounds of induction and exceed the per-fn
budget (60 min); leaving as FOLLOW-UP.

## Final verification status

`Libcrux_ml_kem.Ind_cpa.fst.checked` rebuilds **green** in ~17 s (clean):

```
TOTAL TIME 16773 ms  (rebuild from clean cache; 25 s wall)
Verified module: Libcrux_ml_kem.Ind_cpa
All verification conditions discharged successfully
```

`deserialize_vector` queries with the new per-index lemma:

```
deserialize_vector, 1   succeeded   105 ms   rlimit 0.063 / 800
deserialize_vector, 2   succeeded   202 ms   rlimit 1.273 / 800
```

## R-rule audit

* R1 — no remote push, no PR, no force-push.  ✓
* R2 — no new admits; no `--admit_smt_queries true`; no `admit ()`.  ✓
* R3 — per-fn 60-min budget: deserialize_vector 8 min; deserialize_then_decompress_u **exceeded** (~50 min between investigation, build, hung Z3); both Family A fns un-attempted as a result.  Pre-emptively reverted blocked attempt.
* R4 — rlimit cap 800, no fn pushed > 800.  ✓ (existing `--z3rlimit 800` retained on touched fns)
* R5 — no edits to `src/vector/traits.rs`.  ✓
* R7 — only edited `src/ind_cpa.rs`; no manual edits to `proofs/fstar/extraction/*.fst[i]`.  ✓
* R8 — `python3 hax.py extract` for re-extraction.  ✓
* R9 — real verification, no admit-shuffling; lighthouse genuinely flipped and re-verified.  ✓
* R11 — commit prefixed `agent-mlkem:`.  ✓ (see commit chain below)

## Commit chain

* `be51b4b79` agent-mlkem: Phase F Stream 2 — deserialize_vector lax→panic_free

(Single commit; other 3 fns left as-is with updated FOLLOW-UP for
deserialize_then_decompress_u and unchanged FOLLOW-UPs for
serialize_vector/compress_then_serialize_u.)

## Pattern reusability

The lighthouse pattern is suitable for **Family B without NTT lift**: the
extracted spec function uses `createi`, the loop invariant matches the
spec body except for a `try_into → array_from_fn` shim, and the per-index
goal reduces to `Seq.equal` via a small inner `eq_intro`.  Once
`ntt_vector_u`'s functional ensures is restored (Phase F follow-up),
`deserialize_then_decompress_u` should flip with the same pattern plus
a `Seq.slice_slice` step in the inner lemma.  Family A needs an
unrolling lemma added to `specs/ml-kem` (or inlined in
`ind_cpa.rs`'s hax block) before the same pattern can apply to a
fold-shaped spec.

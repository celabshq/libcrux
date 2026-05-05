# Sprint rollup — 2026-05-05 — `deserialize_then_decompress_u` lax→panic_free attempt

**Branch:** `libcrux-ml-kem-proofs`
**Tip on entry:** `b11ca1033`
**Tip on exit:** `6b5c48c6b` (step 1 only — Bridges unblock)

## What landed

### Step 1 — `Hacspec_ml_kem.Commute.Bridges.poly_to_spec_eq_to_spec_poly_plain` unblocked
Commit `6b5c48c6b`. Added `hax_lib::fstar::after` blocks (interface + body) on
`crate::vector::spec::poly_to_spec` in `libcrux-ml-kem/src/vector.rs` to
re-introduce the verified `Libcrux_ml_kem.Vector.Spec.poly_to_spec_index`
lemma that prior commit `dac5826b0` had added directly to extracted F* (which
were wiped by a subsequent `./hax.py extract`).

Verified: `OTHERFLAGS='--z3refresh' make check/Hacspec_ml_kem.Commute.Bridges.fst → rc=0`.

## What did not land

### Step 2 — `deserialize_then_decompress_u` lax → panic_free
**Reverted.** Two distinct blockers surfaced; both deserve fresh sessions.

**Blocker A (pre-existing, NOT caused by my work):**
`Libcrux_ml_kem.Ind_cpa.fst(891,4-895,10)` — `encrypt_unpacked`'s
`update_at_range ciphertext range tmp0` produces an array of length
`Rust_primitives.Arrays.length ciphertext` and F* must coerce to
`t_Array u8 v_CIPHERTEXT_SIZE`. The Subtyping check times out (canceled at
rlimit 800). Confirmed pre-existing on tip `b11ca1033` BEFORE my step 1 by
reverting and re-extracting — same single error reproduces. Does NOT depend
on Bridges.fst (Ind_cpa.fst has no transitive import). The next-session
prompt's "Bridges blocks Ind_cpa" claim was wrong.

This blocks the **acceptance criterion** `make check/Libcrux_ml_kem.Ind_cpa.fst → rc=0`
regardless of whether step 2 lands. Should be tackled as its own sprint.
F* aborts on this error before reaching any function past line 891 (so I
could not validate step 2 in a clean run).

**Blocker B (step-2 specific):**
With the loop invariant `forall j < i. is_bounded_poly 3328 (Seq.index u_as_ntt j)`,
the per-iteration maintenance check at the cloop fold body (extracted line ~1118)
fails: F* must show that after two `update_at_usize` calls at index `i`,
the new array satisfies the invariant for `j < i+1`. The forall over old
indices and the new index need explicit chaining hints.

I attempted to add explicit asserts inside the loop body (post-`ntt_vector_u`):
```rust
hax_lib::fstar!(r#"
  assert (forall (j: nat). j < v $i ==>
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $u_as_ntt j));
  assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328) (Seq.index $u_as_ntt (v $i)))
"#);
```

Validated by temporarily admitting `encrypt_unpacked`'s body via
`--admit_smt_queries true` to bypass blocker A. Result: 4 errors instead of 2.
The asserts themselves trigger Subtyping failures because Z3 cannot prove
`j < length u_as_ntt` and `v $i < length u_as_ntt`. Need to first establish
`Seq.length u_as_ntt == v $K` and `v $i < v $K`.

Net: this site needs a more careful loop-invariant + post-body proof
structure. The hint inside the body must establish the array-length facts
before referencing `Seq.index`.

## Recommendations for the next session

1. **Address blocker A first** as its own sprint. The `encrypt_unpacked` flake
   may need an explicit `assert (Seq.length ciphertext == v v_CIPHERTEXT_SIZE)`
   immediately after the `Rust_primitives.Hax.repeat` call, or restructuring
   to give Z3 the length-equality as a typed fact. This blocker exists at tip
   `b11ca1033` and pre-dates my step-1 commit.

2. **For blocker B**, the loop-body hint should look like:
   ```rust
   hax_lib::fstar!(r#"
     assert (Seq.length $u_as_ntt == v $K);
     assert (v $i < v $K);
     assert (forall (j: nat). j < v $i ==>
       Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328)
         (Seq.index $u_as_ntt j));
     assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (sz 3328)
       (Seq.index $u_as_ntt (v $i)))
   "#);
   ```
   Or, alternatively, switch from `cloop! .. .chunks_exact(..).enumerate()` to
   plain `for i in 0..K { let u_bytes = &ciphertext[i*BLK..(i+1)*BLK]; ... }`
   to match the working pattern in `sample_vector_cbd_then_ntt` at line ~272
   (which uses `for i in 0..K { ... re_as_ntt[i] = ...; ntt_binomially_sampled_ring_element(&mut re_as_ntt[i]); }`
   and works without explicit body hints).

## Time accounting
~110 min total (over the 60-90 min budget). Step 1 took ~20 min;
step 2 attempts consumed the rest, half on a too-long F* run that aborted
in `encrypt_unpacked` (blocker A) before reaching the target function.

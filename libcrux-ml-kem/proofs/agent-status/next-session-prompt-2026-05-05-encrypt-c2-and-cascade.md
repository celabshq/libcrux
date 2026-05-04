# Next-session prompt — close encrypt_c2 + cascade up

## Branch start

Tip: `libcrux-ml-kem-opaque-bounds-spike` (worktree
`/Users/karthik/libcrux-opaque-spike`), HEAD `5341aaec2`.  Optionally
merge into `libcrux-ml-kem-proofs` first if the spike is to be the new
mainline; otherwise continue on the spike branch.

## Read first (15-20 min, do not skip)

1. `proofs/agent-status/sprint-2026-05-04-opaque-bounds-spike-rollup.md`
   — context for this session.  §"What did NOT land", §"Next-session
   unblocks" are load-bearing.
2. MEMORY.md entries:
   * `feedback_dual_smtpat_opaque_atom` — the spike's main finding;
     informs whether new bound predicates need intros.
   * `feedback_grep_make_output`, `feedback_use_fstar_mcp` — token
     hygiene.
   * `feedback_proof_debug_budget` — 30-60 min cap per fn.
3. `git show 501888222 e01ad3b73 d2b34d7dc` — the spike's three site
   commits.  Especially d2b34d7dc^^..d2b34d7dc shows the encrypt_c1
   body shape (Classical.forall_intro of elim_nat + vector_higher
   widening + vector_intro fold) — this is the template for encrypt_c2.

## Goal

Close the remaining 1 of 3 encrypt_cN sites and propagate the matrix
bound up through the encapsulate/decapsulate cascade.  Two independent
sub-tasks (do them in this order):

  * **Sub-task 1 (~30 min):** strengthen
    `deserialize_then_decompress_message`'s ensures, flip `encrypt_c2`
    lax → panic_free.
  * **Sub-task 2 (~1-1.5h):** cascade
    `is_bounded_polynomial_matrix(3328, public_key.A)` requires up
    through `Ind_cca::encapsulate`, `Ind_cca::decapsulate`,
    `Ind_cca::instantiations` (3 backends), `mlkem{512,768,1024}::encapsulate`.

## Sub-task 1 — encrypt_c2 (close)

### Step 1.1 — strengthen producer ensures (~10 min)

Edit `src/serialize.rs::deserialize_then_decompress_message`.  Current
ensures (extracted in `Libcrux_ml_kem.Serialize.fsti` around the
`val deserialize_then_decompress_message` line):

  ```fstar
  ensures fun result ->
    Libcrux_ml_kem.Vector.Spec.poly_to_spec result ==
    Hacspec_ml_kem.Serialize.deserialize_then_decompress_message serialized
  ```

Add a conjunct:
  `crate::polynomial::spec::is_bounded_poly(3328, result)`

The values in `result` are 0 or 1664 by construction (decompress_1 of
binary input).  Sound at any `b ≥ 1664`; we use 3328 to match
`compute_ring_element_v`'s precondition.

The body's discharge: the underlying loop sets each lane via
`decompress_1` which produces `i16` in `{0, 1664}`.  Likely needs:
  * No body change, or
  * One `is_bounded_poly_higher` call, or
  * A small `assert (...)` hint near the loop's exit.

If the discharge takes more than 30 min, drop to a `panic_free`-with-
admit_smt_queries on this fn JUST to unblock encrypt_c2.  Note in
FOLLOW-UP and continue.

### Step 1.2 — re-apply encrypt_c2 flip (~15 min)

Restore the encrypt_c2 lax → panic_free + new requires + body
helpers.  The exact shape is in `git show d2b34d7dc^` (the parent
commit's version of `src/ind_cpa.rs` had the body before I reverted
it for the partial land).  Specifically:

  * **Requires** (add to encrypt_c2):
    ```rust
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, t_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, r_as_ntt)
    & crate::polynomial::spec::is_bounded_poly(3328, error_2)
    ```
  * **Options:** `--z3rlimit 400 --ext context_pruning --split_queries always`
  * **Verification status:** `panic_free`
  * **Body:** insert before `compute_ring_element_v` call:
    ```rust
    hax_lib::fstar!(
        r#"
        assert (Seq.length $t_as_ntt == v $K);
        assert (Seq.length $r_as_ntt == v $K);
        let aux_t (i: nat) : Lemma (requires i < Seq.length $t_as_ntt)
                                   (ensures i < v $K ==>
                                            Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly
                                              (sz 3328) (Seq.index $t_as_ntt i)) =
          if i < v $K then
            Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_elim_nat
              $K #$:Vector (sz 3328) $t_as_ntt i
        in
        Classical.forall_intro (Classical.move_requires aux_t);
        let aux_r (i: nat) : Lemma (requires i < Seq.length $r_as_ntt)
                                   (ensures i < v $K ==>
                                            Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly
                                              (sz 3328) (Seq.index $r_as_ntt i)) =
          if i < v $K then
            Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_elim_nat
              $K #$:Vector (sz 3328) $r_as_ntt i
        in
        Classical.forall_intro (Classical.move_requires aux_r)"#
    );
    ```

### Step 1.3 — thread encrypt_unpacked (~5 min)

`encrypt_unpacked` already has matrix-bound requires (added in
d2b34d7dc).  Add the t_as_ntt vector-bound requires too:

  ```rust
  & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &public_key.t_as_ntt)
  ```

`encrypt::<...>(public_key: &[u8], ...)` (the unpacking variant) calls
`build_unpacked_public_key` which already produces both bounds — so
encrypt's requires don't need the vector bound (it's discharged
locally).  But `encrypt_unpacked` is also called from
`Ind_cca::Unpacked` (sub-task 2's domain) — that's where the next
cascade step kicks in.

### Acceptance for sub-task 1

```
$ grep -c "verification_status(lax)" src/ind_cpa.rs
1     # only deserialize_then_decompress_u remains (out-of-scope thread)
$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.{Serialize,Ind_cpa}.fst
rc=0
```

### Hard stop sub-task 1

If sub-task 1 takes > 60 min, commit any partial progress and STOP
before starting sub-task 2.  Sub-task 2 is independent enough to skip.

## Sub-task 2 — cascade up

**Note on terminology.**  `src/ind_cca.rs` is already lax-free (0 sites
per spike rollup acceptance check).  Sub-task 2 does NOT add new lax;
it **threads strengthened requires** through the already-panic_free
fns (encapsulate/decapsulate/instantiations/mlkem*::encapsulate) so
that each level can discharge its callee's new
`is_bounded_polynomial_matrix(3328, public_key.A)` precondition.

The risk is that re-verifying these bodies under the new requires-shape
exposes Z3 brittleness (hint replays missing a quantifier, etc.) —
not new lax.  If you hit that, prefer a small explicit lemma call
inside the body (e.g. an `assert` or `lemma_*_elim` invocation) over
an admit/lax flip; surface to user before flipping anything.

### Pre-flight: pre-existing Ind_cca.Unpacked.fst body failure

Per the spike rollup §"Cascade up", `Libcrux_ml_kem.Ind_cca.Unpacked.fst`'s
.fst body was failing PRE-spike (no baseline `.checked`) at line 151
`serialize_public_key_mut` requires.  The cascade work touches this
file's encapsulate/decapsulate → if you can't get this file's body
verifying first, sub-task 2 will drop into pre-existing failures
mid-cascade.  Before starting:

  ```
  $ rm -f /Users/karthik/libcrux-opaque-spike/.fstar-cache/checked/Libcrux_ml_kem.Ind_cca.Unpacked.fst.checked
  $ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.Ind_cca.Unpacked.fst
  ```

If rc=2 (pre-existing failure), it's NOT yours to fix in this session
— mark sub-task 2 as PARTIAL after the fsti-only updates and stop.

### Cascade plan (top-down)

The cascade adds
`is_bounded_polynomial_matrix(3328, public_key.ind_cpa_public_key.A) /\
 is_bounded_polynomial_vector(3328, public_key.ind_cpa_public_key.t_as_ntt)`
requires (or whatever subset is consumed at each level) at:

  1. `src/ind_cca.rs::Unpacked::encapsulate` — calls
     `ind_cpa::encrypt_unpacked`.  Add both bound requires.
  2. `src/ind_cca.rs::Unpacked::decapsulate` — same chain via
     `encrypt_unpacked` again (re-encrypt).
  3. `src/ind_cca/instantiations.rs` — 3 backends × 2 ops, each
     forwards to (1)/(2).  Add same requires.
  4. `src/mlkem{512,768,1024}.rs::encapsulate` (unpacked variant) —
     forwards to instantiations.  Add same requires.

For each level, after editing:
  * `./hax.py extract` (~30s)
  * `make check/Libcrux_ml_kem.{level-module}.fsti` (cheap)
  * `OTHERFLAGS='--z3refresh' make check/...{level-module}.fst` (slow)

### Hard stop sub-task 2

  * Cascade arrives at a producer site that doesn't trivially provide
    the bound (some baseline weakness exposed) — STOP, document FOLLOW-UP.
  * Per-fn budget exceeded on any one level — STOP, mark FOLLOW-UP.
  * Total session > 2.5h regardless — STOP, rollup.

## Commit hygiene

  * One commit per logical step: producer ensures strengthen, encrypt_c2
    flip, each cascade level.
  * Commit messages: `agent-mlkem: <verb> <fn> ...` (match prior).
  * Rollup at
    `proofs/agent-status/sprint-YYYY-MM-DD-encrypt-c2-cascade-rollup.md`.

## Acceptance — full session

```
$ grep -c "verification_status(lax)" src/ind_cpa.rs src/ind_cca.rs
src/ind_cpa.rs:1   # only deserialize_then_decompress_u (out of scope)
src/ind_cca.rs:0   # unchanged from spike entry; cascade adds requires, not lax

$ OTHERFLAGS='--z3refresh' make check/Libcrux_ml_kem.{Serialize,Ind_cpa,Ind_cca,Ind_cca.Unpacked}.fst
rc=0    # OR rc=2 only on Ind_cca.Unpacked.fst with the pre-existing
        # serialize_public_key_mut failure (independent of this work).
```

A partial that closes only sub-task 1 is also a valid land — the
encrypt_c2 flip alone closes 2/3 ind_cpa lax sites and is a clean
incremental commit.

## Key pointers

  * Spike rollup: `proofs/agent-status/sprint-2026-05-04-opaque-bounds-spike-rollup.md`.
  * encrypt_c1 body template: `git show d2b34d7dc -- src/ind_cpa.rs` (lines around 730-790).
  * Reverted encrypt_c2 body shape: `git show d2b34d7dc^^ -- src/ind_cpa.rs` (~lines 810-830 had the aux_t/aux_r helpers).
  * elim_nat + vector_higher lemmas: `src/polynomial.rs` (in spec module + raw F* via `fstar::after`).
  * compute_ring_element_v requires (target shape):
    `proofs/fstar/extraction/Libcrux_ml_kem.Matrix.fsti` near `val compute_ring_element_v`.

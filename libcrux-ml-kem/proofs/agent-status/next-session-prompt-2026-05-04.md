# Next-session prompt — ML-KEM proof sprint 2026-05-04

**Branch:** `libcrux-ml-kem-proofs`  
**Tip on entry:** `3c73d1a96`

---

## What was done this session

### Spec rewrite — serialize_public_key createi form
- **`specs/ml-kem/src/serialize.rs`** (`ef703d544`): rewrote `serialize_public_key` from a loop+copy_from_slice body to `createi(|k| if k < RANK*384 { byte_encode(...)[k%384] } else { seed[k-RANK*384] })`. Cargo tests pass (47 tests green). Hax extraction updated `Hacspec_ml_kem.Serialize.fst.checked`.

### Bridge lemmas proved
- **`Hacspec_ml_kem.Commute.Serialize.fst`** (`f70595b23`, `084c21741`):
  - `serialize_secret_key_chunk_eq` — `Seq.slice (serialize_secret_key K T v) (j*384) ((j+1)*384) == byte_encode(v[j], 12)` for each `j < K`.  Proof: `lemma_index_slice` + nat→usize hint `assert (lo+m == v (mk_usize (lo+m)))` so `createi_lemma` SMTPat fires.
  - `serialize_secret_key_all_chunks_eq` — ∀j forall version.
  - `serialize_public_key_vector_eq` — `Seq.slice (serialize_public_key K EK_SIZE t seed) 0 (K*384) == serialize_secret_key K (K*384) t`. Proof: if-branch of createi fires for m < K*384.
  - `serialize_public_key_seed_eq` — `Seq.slice (serialize_public_key K EK_SIZE t seed) (K*384) EK_SIZE == Seq.slice seed 0 32`. Proof: else-branch fires for m >= K*384.
  - All four lemmas verified at z3rlimit 300, make exits 0.

### Pre-existing F* fragility fixed
- **`libcrux-ml-kem/src/ind_cpa.rs`** (`3c73d1a96`): `generate_keypair_unpacked`, `encrypt_unpacked`, `encrypt` — changed `requires is_rank(K) && eta1_randomness_size(K) == ...` to `requires is_rank(K).to_prop() & (...).to_prop()`. Root cause: the new `createi`-based `serialize_public_key` spec introduces `createi_lemma` SMTPat axioms that caused Z3 "incomplete quantifiers" on the `eta1_randomness_size` precondition check when the boolean `&&` was used (F* doesn't propagate the left side as a hypothesis in boolean `&&`). Logical `l_and` via `.to_prop() &` fixes it. `make check/Libcrux_ml_kem.Ind_cpa.fst` exits 0.

---

## Blocked work — wiring serialize_public_key to ind_cpa

Attempted to wire `serialize_public_key`'s ensures in `ind_cpa.rs` using admitted ensures on `serialize_vector` and `serialize_public_key_mut`. **Reverted due to Z3 quantifier pollution.**

### Root cause of the block
Adding admitted ensures to `serialize_vector` and `serialize_public_key_mut` that reference `Hacspec_ml_kem.Serialize.serialize_public_key` (a `createi(if-else)` function) creates global Z3 axioms with `createi_lemma` SMTPat triggers. These axioms pollute the context for later `panic_free` functions (specifically `generate_keypair_unpacked`) and cause "incomplete quantifiers" on precondition checks. Even after fixing the `is_rank` propagation issue (commit `3c73d1a96`), adding the ensures reintroduces the problem.

### The correct wiring approach (next session)

The impl-side wiring requires two things:

**Step 1 — `serialize_vector` body proof** (`ind_cpa.rs`, currently lax)

The body has a post-loop `eq_intro out ssk` that fails because Z3 can't instantiate the universally-quantified loop invariant `∀j < K, Seq.slice out (j*384) ((j+1)*384) == byte_encode(...)` for all byte positions `m`.

Fix: replace the post-loop `hax_lib::fstar!` block with an explicit per-byte proof:
```fstar
let vts = ${vector_to_spec::<K, Vector>} $K $key in
let ssk = Hacspec_ml_kem.Serialize.serialize_secret_key $K ($K *! sz 384) vts in
Hacspec_ml_kem.Commute.Serialize.serialize_secret_key_all_chunks_eq $K vts;
let aux (m: nat) : Lemma (m < Seq.length ssk ==> Seq.index $out m == Seq.index ssk m) =
  if m < Seq.length ssk then begin
    let j = m / v $BYTES_PER_RING_ELEMENT in
    let r = m % v $BYTES_PER_RING_ELEMENT in
    Seq.lemma_index_slice $out (j * v $BYTES_PER_RING_ELEMENT) ((j + 1) * v $BYTES_PER_RING_ELEMENT) r;
    Seq.lemma_index_slice ssk (j * v $BYTES_PER_RING_ELEMENT) ((j + 1) * v $BYTES_PER_RING_ELEMENT) r
  end in
Classical.forall_intro aux;
Seq.lemma_eq_intro $out ssk
```
Also change `serialize_vector`'s ensures to use `serialize_secret_key` (not `serialize_secret_key_into`):
```rust
#[hax_lib::ensures(|()| fstar!(r#"${out}_future ==
    Hacspec_ml_kem.Serialize.serialize_secret_key $K ($K *! sz 384)
      (${vector_to_spec::<K, Vector>} $K $key)"#))]
```
Change status from `lax` to `panic_free`.

**Step 2 — `update_at_range_result_lemma`** (needed by `serialize_public_key_mut` body proof)

`serialize_public_key_mut` calls `serialize_vector` which returns the mutated slice via `update_at_range`. To prove `Seq.slice result 0 (K*384) == ssk`, need a lemma:
```fstar
val update_at_range_result_lemma
  (s: t_Slice 't) (i: t_Range usize) (x: t_Slice 't)
  : Lemma
    (requires Seq.length x == v i.f_end - v i.f_start)
    (ensures Seq.slice (update_at_range s i x) (v i.f_start) (v i.f_end) == x)
```
Add as `assume val` to a NEW file `Spec.Utils.Extra.fst/fsti` in `libcrux-ml-kem/proofs/fstar/spec/` (do NOT touch `Spec.Utils.fsti` — it has 19 importers and adding a new SMTPat could cascade). Similarly add `update_at_range_from_result_lemma` for the seed-copy step.

**Step 3 — `serialize_public_key_mut` body proof + ensures**

With `serialize_vector`'s ensures providing `serialized[0..K*384] == ssk` and `update_at_range_result_lemma` bridging the mutation, prove:
```fstar
// after serialize_vector: Seq.slice serialized 0 (K*384) == ssk (from update_at_range_result_lemma + serialize_vector ensures)
// after seed copy: Seq.slice serialized (K*384) EK_SIZE == Seq.slice seed 0 32 (from update_at_range_from_result_lemma)
// from bridge lemmas: spec_pk[0..K*384] = ssk, spec_pk[K*384..EK_SIZE] = seed[0..32]
// therefore: serialized == spec_pk
```
Change `serialize_public_key_mut`'s ensures to `${serialized}_future == Hacspec_ml_kem.Serialize.serialize_public_key ...` and status to `panic_free`.

**Step 4 — `serialize_public_key` closes trivially** once `serialize_public_key_mut` has a REAL (not admitted) ensures.

### Why the non-admitted route avoids Z3 pollution

The Z3 pollution occurs because ADMITTED ensures become global axioms with universal quantifiers. A REAL body proof does NOT add these axioms — the postcondition is proved inline and the axiom is never broadcast globally. So once steps 1–3 use real body proofs (not admits), step 4's verification will be clean.

---

## Other remaining work

### `serialize_unpacked_secret_key` (Stream 2.2)
After `serialize_public_key` is verified, `serialize_unpacked_secret_key` (lax) can be flipped. It calls:
- `serialize_public_key` (panic_free after Step 4 above)
- `serialize_vector` (panic_free after Step 1 above)

Its functional ensures connects to the Hacspec `ind_cpa::serialize_unpacked_secret_key` spec. This is a mechanical cascade once the serialize chain is verified.

### Audit debt — 3 `--admit_smt_queries true` in `Ind_cca.Unpacked.fst{i}`
From the May 3 session rollup. See `sprint-2026-05-03-rollup.md` for details. The fix requires:
1. `keys_from_private_key` body: assert that `deserialize_vector`'s ensures flows through record updates to `f_secret_as_ntt`
2. `decapsulate` requires: stabilize `is_bounded_polynomial_vector` forall in the Z3 context (might need `assert (is_bounded_polynomial_vector ...)` hint)

### `deserialize_then_decompress_u` loop invariant (U-7)
The `ntt_vector_u` ensures IS already stated (body admitted, commit `56f3eea01`). The loop invariant proof needs to track the NTT spec through the `ntt_vector_u` call. This is the real work: maintaining `vector_to_spec(u[0..i]) == map ntt (vector_to_spec(decompress u))` across each loop iteration.

---

## Key file paths
- Bridge lemmas: `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Serialize.fst`
- Spec: `specs/ml-kem/src/serialize.rs`
- Impl: `libcrux-ml-kem/src/ind_cpa.rs`
- Spec.Utils: `libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.fst/fsti` (19 importers — touch carefully)
- New lemma file target: `libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.Extra.fst/fsti`

## Make commands
```bash
# Verify bridge lemmas
cd specs/ml-kem/proofs/fstar/commute && make check/Hacspec_ml_kem.Commute.Serialize.fst > /tmp/commute.log 2>&1

# Verify Ind_cpa
cd libcrux-ml-kem/proofs/fstar/extraction && make check/Libcrux_ml_kem.Ind_cpa.fst > /tmp/ind_cpa.log 2>&1

# Verify Ind_cca.Unpacked (regression check)
cd libcrux-ml-kem/proofs/fstar/extraction && make check/Libcrux_ml_kem.Ind_cca.Unpacked.fst > /tmp/ind_cca.log 2>&1
```

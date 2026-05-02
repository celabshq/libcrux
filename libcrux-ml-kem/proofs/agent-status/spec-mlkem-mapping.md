# Spec.MLKEM → Hacspec_ml_kem mapping table — Lane E

**Date:** 2026-05-02
**Branch:** `libcrux-ml-kem-proofs`
**Source enumeration:**
```
grep -rhEo "Spec\.MLKEM\.[A-Za-z_][A-Za-z0-9_]*" \
  libcrux-ml-kem/src libcrux-ml-kem/proofs/fstar/extraction \
  | sort -u
```
**Distinct symbols:** 49 (29 mapped, 20 unmapped)
**Total cite count:** 915 (417 in `src/*.rs`, 498 in `extraction/*.fst[i]`)

Per-file `src/` distribution (Lane E sweep target):

| File | Cites |
|---|---|
| `src/ind_cca/instantiations/avx2.rs` | 147 |
| `src/ind_cca/instantiations.rs` | 85 |
| `src/ind_cpa.rs` | 51 |
| `src/ind_cca.rs` | 48 |
| `src/ind_cca/multiplexing.rs` | 40 |
| `src/serialize.rs` | 17 |
| `src/mlkem512.rs` | 3 |
| `src/mlkem1024.rs` | 3 |
| `src/ntt.rs` | 2 (already commented out) |
| `src/sampling.rs` | 1 |
| `src/ntt.rs~` | 2 (stale backup, ignore) |

> **Note:** `src/mlkem768.rs` does not appear — likely no cites at top level.
> Will verify during Phase 2.

---

## Mapped symbols (29)

### Constants — `Hacspec_ml_kem.Parameters` (rank-generic, identical name & shape)

| Spec.MLKEM symbol | Hacspec replacement | Arg-shape note |
|---|---|---|
| `Spec.MLKEM.is_rank` | `Hacspec_ml_kem.Parameters.is_rank` | identical (rank → bool) |
| `Spec.MLKEM.v_SHARED_SECRET_SIZE` | `Hacspec_ml_kem.Parameters.v_SHARED_SECRET_SIZE` | identical (constant) |
| `Spec.MLKEM.v_CPA_KEY_GENERATION_SEED_SIZE` | `Hacspec_ml_kem.Parameters.v_CPA_KEY_GENERATION_SEED_SIZE` | identical (constant) |
| `Spec.MLKEM.v_H_DIGEST_SIZE` | `Hacspec_ml_kem.Parameters.Hash_functions.v_H_DIGEST_SIZE` | identical (constant) |

### Constants — `Hacspec_ml_kem.Parameters` (rank-arg now positional)

> In Spec.MLKEM these are macro-style: `Spec.MLKEM.v_X K`.  In Hacspec
> they are functions taking `rank: usize` (lowercase, no `v_` prefix).

| Spec.MLKEM symbol | Hacspec replacement | Arg-shape note |
|---|---|---|
| `Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE K` | `Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size K` | rank-arg positional |
| `Spec.MLKEM.v_RANKED_BYTES_PER_RING_ELEMENT K` | `Hacspec_ml_kem.Parameters.ranked_bytes_per_ring_element K` | rank-arg positional |
| `Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE K` | `Hacspec_ml_kem.Parameters.cpa_public_key_size K` | rank-arg positional |
| `Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE K` | `Hacspec_ml_kem.Parameters.cpa_private_key_size K` | rank-arg positional |
| `Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE K` | `Hacspec_ml_kem.Parameters.cpa_ciphertext_size K` | rank-arg positional |
| `Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE K` | `Hacspec_ml_kem.Parameters.cca_private_key_size K` | rank-arg positional |
| `Spec.MLKEM.v_C1_BLOCK_SIZE K` | `Hacspec_ml_kem.Parameters.c1_block_size K` | rank-arg positional |
| `Spec.MLKEM.v_C1_SIZE K` | `Hacspec_ml_kem.Parameters.c1_size K` | rank-arg positional |
| `Spec.MLKEM.v_C2_SIZE K` | `Hacspec_ml_kem.Parameters.c2_size K` | rank-arg positional |
| `Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR K` | `Hacspec_ml_kem.Parameters.vector_u_compression_factor K` | rank-arg positional |
| `Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR K` | `Hacspec_ml_kem.Parameters.vector_v_compression_factor K` | rank-arg positional |
| `Spec.MLKEM.v_ETA1 K` | `Hacspec_ml_kem.Parameters.eta1 K` | rank-arg positional |
| `Spec.MLKEM.v_ETA2 K` | `Hacspec_ml_kem.Parameters.eta2 K` | rank-arg positional |
| `Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE K` | `Hacspec_ml_kem.Parameters.eta1_randomness_size K` | rank-arg positional |
| `Spec.MLKEM.v_ETA2_RANDOMNESS_SIZE K` | `Hacspec_ml_kem.Parameters.eta2_randomness_size K` | rank-arg positional |
| `Spec.MLKEM.v_IMPLICIT_REJECTION_HASH_INPUT_SIZE K` | `Hacspec_ml_kem.Parameters.implicit_rejection_hash_input_size K` | rank-arg positional |

### Functions — direct rename (same module path or known Hacspec landing)

| Spec.MLKEM symbol | Hacspec replacement | Arg-shape note |
|---|---|---|
| `Spec.MLKEM.byte_decode d v` | `Hacspec_ml_kem.Serialize.byte_decode (d*32) (d*256) v d` | **shape changes**: Hacspec takes explicit `(D32, D256)` size args plus the byte array and `d`; Spec.MLKEM takes only `(d, v)` |
| `Spec.MLKEM.byte_encode d p` | `Hacspec_ml_kem.Serialize.byte_encode (d*32) (d*256) p d` | **shape changes**: same as `byte_decode` |
| `Spec.MLKEM.sample_poly_cbd eta bytes` | `Hacspec_ml_kem.Sampling.sample_poly_cbd (eta*64) (eta*512) eta bytes` | **shape changes**: Hacspec takes `(ETA64, ETA512, eta, bytes)` |
| `Spec.MLKEM.sample_vector_cbd_then_ntt #K seed ds` | `Hacspec_ml_kem.Ind_cpa.sample_vector_cbd_then_ntt K eta seed ds` | **shape changes**: implicit `K` → positional; Hacspec also takes `eta` positionally; returns just the vector (not vector × ds) |
| `Spec.MLKEM.vector_decode_12 #K v` | `Hacspec_ml_kem.Serialize.vector_decode_12_ K v` | **trailing underscore in Hacspec name** |

### Functions — likely-equivalent serialize family

> These are not literal renames but functional equivalents.  Manual review needed
> per cite to confirm pointwise equality holds (the impl side may compose them
> differently than the spec does).  Mark as **review** in commit messages.

| Spec.MLKEM symbol | Hacspec replacement | Arg-shape note |
|---|---|---|
| `Spec.MLKEM.compress_then_encode_message p` | `Hacspec_ml_kem.Serialize.compress_then_serialize_message p` | identical signature; review pointwise equality |
| `Spec.MLKEM.compress_then_encode_u #K v du` | `Hacspec_ml_kem.Serialize.compress_then_serialize_u K v_U_SIZE v du` | needs explicit `v_U_SIZE` const-arg |
| `Spec.MLKEM.compress_then_encode_v v_size v dv` | `Hacspec_ml_kem.Serialize.compress_then_serialize_v v_V_SIZE v dv` | identical shape |
| `Spec.MLKEM.decode_then_decompress_message s` | `Hacspec_ml_kem.Serialize.deserialize_then_decompress_message s` | identical |
| `Spec.MLKEM.decode_then_decompress_v s dv` | `Hacspec_ml_kem.Serialize.deserialize_then_decompress_v s dv` | identical |

---

## Unmapped symbols (20) — agents must skip per R12

### No Hacspec equivalent exists yet

| Spec.MLKEM symbol | Reason | Suggested follow-up |
|---|---|---|
| `Spec.MLKEM.byte_decode_then_decompress d v` | Per-poly compose; only `deserialize_then_decompress_v` (per ring elt) and `_u` (per vector) exist in Hacspec.  May be inline-able by composing `byte_decode` + `decompress`. | Add `Hacspec_ml_kem.Serialize.byte_decode_then_decompress` helper or inline |
| `Spec.MLKEM.compress_then_byte_encode d p` | Per-poly compose; same situation as above (only `compress_then_serialize_message`/`_u`/`_v` exist). | Add helper or inline |
| `Spec.MLKEM.coerce_vector_12 v` | Spec-only sanitisation function; no Hacspec analogue. | Likely needs `Hacspec_ml_kem.Serialize.coerce_vector_12_` |
| `Spec.MLKEM.vector_encode_12 #K v` | No `vector_encode_12` in Hacspec; closest is `Hacspec_ml_kem.Serialize.serialize_secret_key K v_DK_PKE_SIZE v` (allocating wrapper) but signature requires `v_DK_PKE_SIZE = K * 384`. | Rename or wrap `serialize_secret_key`; per R12 do not auto-substitute |
| `Spec.MLKEM.poly_ntt p` | Closest match `Hacspec_ml_kem.Ntt.ntt p` but the only cite-sites in `src/ntt.rs` are already commented out (lines 521, 561). | Trivially substitutable but skip per R12 unless approved |
| `Spec.MLKEM.polynomial` | Type alias (`t_Array t_FieldElement 256`); not a function. Inline expansion required. | Type substitution, not name substitution |
| `Spec.MLKEM.polynomial_d d` | Type alias for d-bit-bounded polynomial; not a function. | Type substitution |
| `Spec.MLKEM.matrix K` | Type alias (`t_Array (t_Array poly K) K`); not a function. | Type substitution |
| `Spec.MLKEM.matrix_A_as_ntt_i sample` | Helper used to build matrix from sampled NTT; no Hacspec counterpart. | Add helper to `Hacspec_ml_kem.Matrix` |
| `Spec.MLKEM.sample_matrix_A_ntt #K seed` | Hacspec has `Hacspec_ml_kem.Matrix.sample_matrix_A K seed transpose` but: (a) takes explicit `transpose: bool`, (b) does NOT fuse the NTT (FIPS-203 spec samples in NTT domain). | Confirm semantic equivalence or add NTT-fused helper |
| `Spec.MLKEM.sample_vector_cbd2 #K prf bytes` | Hacspec has only `sample_vector_cbd K eta seed ds` (rank-generic, eta positional).  The `cbd2` specialisation isn't a separate name. | Likely substitutable as `sample_vector_cbd K 2 ...` but signature is different (PRF vs seed+ds), needs review |
| `Spec.MLKEM.sample_vector_cbd1_prf_input #K seed ds i` | PRF-input synthesis helper; no direct Hacspec analogue. | Add helper or inline at call sites |
| `Spec.MLKEM.sample_vector_cbd2_prf_input #K seed ds i` | Same as above. | Add helper or inline |
| `Spec.MLKEM.v_PRFxN K size prf_inputs` | Vector-PRF batched helper; Hacspec models the PRF inline. | No direct Hacspec equivalent |
| `Spec.MLKEM.ind_cca_unpack_public_key K pk` | Hacspec has `ind_cca_unpack_*_keypair`/`encapsulate`/`decapsulate` family but **shape differs** (returns Result, no `valid` flag). | Migration changes the assertion structure, not just the name |
| `Spec.MLKEM.ind_cpa_decrypt K dk ct` | Hacspec has `Hacspec_ml_kem.Ind_cpa.decrypt params dk ct` but **shape differs** (params record vs K, no `valid` flag, returns array directly). | Migration is structural, not lexical |
| `Spec.MLKEM.ind_cpa_generate_keypair_unpacked K seed` | Hacspec has `Hacspec_ml_kem.Ind_cpa.generate_keypair_unpacked params seed` but **shape differs** (params record). | Migration is structural |
| `Spec.MLKEM.Instances.mlkem512_*`, `Spec.MLKEM.Instances.mlkem768_*`, `Spec.MLKEM.Instances.mlkem1024_*` (× 3 functions × 3 levels = up to 9 cites) | Hacspec has rank-generic `Hacspec_ml_kem.Ind_cca.{generate_keypair, encapsulate, decapsulate}` taking `params: t_MlKemParams`. | Substitute as `Hacspec_ml_kem.Ind_cca.<fn> v_ML_KEM_512_ ...` etc., but **return shape changes** (Result-typed, no `((x,y), valid)` tuple). |

---

## Aggregate

| Category | Count |
|---|---|
| Mapped — direct rename, identical shape | 5 |
| Mapped — rank-arg positional (constants → fns) | 16 |
| Mapped — known shape change (review per cite) | 5 + 3 = 8 |
| **Total mapped** | **29** |
| Unmapped — no Hacspec equivalent (skip per R12) | 14 |
| Unmapped — type aliases (substitution shape unknown) | 3 |
| Unmapped — Result-typed Hacspec, structural shift | 3 |
| **Total unmapped** | **20** |

---

## Worktree-agent dispatch (Phase 2 plan)

Lane partitioning (preliminary; adjust based on review feedback):

| Lane | Files | Approx cites in src/ |
|---|---|---|
| E1 | `src/sampling.rs` + `src/mlkem512.rs` + `src/mlkem768.rs` + `src/mlkem1024.rs` | 7 |
| E2 | `src/serialize.rs` | 17 |
| E3 | `src/ind_cpa.rs` | 51 |
| E4 | `src/ind_cca.rs` | 48 |
| E5 | `src/ind_cca/multiplexing.rs` | 40 |
| E6 | `src/ind_cca/instantiations.rs` | 85 |
| E7 | `src/ind_cca/instantiations/avx2.rs` | 147 |

`src/ntt.rs~` is a stale backup — the orchestrator will delete it
unless the user objects.

`src/ind_cca/instantiations/{neon,portable}.rs` were not enumerated
because their cite counts are 0 (or absorbed into the `instantiations.rs`
parent).  Will verify in Phase 2.

---

## R12 self-audit

- 20 of 49 distinct symbols are unmapped.  Agents will leave these
  cites untouched and record them in per-lane `lane-E<N>-unmapped.md`.
- The 8 "known shape change" mapped rows below the inventory may
  produce post-substitution build failures; per the prompt's
  out-of-scope clause, agents will flag (not redesign) such failures.

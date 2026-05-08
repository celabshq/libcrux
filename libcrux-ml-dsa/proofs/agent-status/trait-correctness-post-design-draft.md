# Trait Correctness-Post Design Draft — 2026-05-04

Status: DRAFT — week-2 agents should refine before implementing.

Grounded in:
- `src/simd/traits.rs` + `src/simd/traits/specs.rs` (current ML-DSA state)
- `~/libcrux-trait-opacify/libcrux-ml-kem/src/vector/traits.rs` (ML-KEM reference patterns)
- `specs/ml-dsa/proofs/fstar/extraction/Hacspec_ml_dsa.{Ntt,Encoding,Polynomial}.fst`

---

## Key architectural decisions (from ML-KEM study)

### 1. Serialization: `BitVecEq` generic-over-N, NOT direct Hacspec cite at trait level

ML-KEM uses `serialize_post_N (#n: usize) (d: nat) (input: t_Array i16 n) (output: t_Slice u8)`
backed by `BitVecEq.int_t_array_bitwise_eq`. Generic over array size.

**ML-DSA should mirror this**: `bit_pack_post_N (#n: usize)` and `simple_bit_pack_post_N (#n: usize)`.
- At n=8 (trait level): post over the 8-lane SIMD unit
- At n=256 (poly level): same predicate, proving whole-ring correctness
- Hacspec bridge (`hacspec_simple_bit_pack 256 w == result`) is a separate lemma in `Commute.Chunk`

This is **week-4 pre-work** (requires verifying `BitVecEq` is on the ML-DSA F* include path).

### 2. Montgomery / reduce / shift_left_then_reduce: introduce `mod_q_eq` to hide raw `%`

ML-KEM uses `Hacspec_ml_kem.ModQ.mod_q_eq (a: int) (b: int)` which wraps `a % 3329 == b % 3329`.
Raw `% 3329` never appears above the trait — avoids non-linear SMT leak (per feedback).

**ML-DSA should add** `mod_q_eq (a b: int) : prop = a % 8380417 == b % 8380417`
in `specs.rs` F* block (local first, promote to `Spec.MLDSA.Math.fst` after shape is final).

### 3. Zero / from / to: direct array equality in ensures, no opaque predicate

ML-KEM: `#[ensures(|result| result.repr() == [0i16; 16])]` — just a direct equation.
ML-DSA should do the same (currently these have no ensures at all).

### 4. NTT: `to_spec_poly` lift + whole-ring opaque post

ML-KEM has per-layer NTT step methods (not a monolithic `ntt`). ML-DSA has one
`Operations::ntt(&mut [Self; 32])` taking the whole ring element. So ML-DSA
needs a `to_spec_poly` lift:
```fstar
let to_spec_poly (simd_units: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : t_Array i32 (mk_usize 256) =
  Seq.init (mk_usize 256) (fun k ->
    Seq.index (Seq.index simd_units (k / 8)) (k % 8))
```

---

## Per-method design

### Group 1 — Trivial identity (direct equality, no opaque predicate)

| Method | Proposed ensures | Notes |
|---|---|---|
| `zero` | `future(out).repr() == [0i32; 8]` | Direct, no predicate |
| `from_coefficient_array` | `future(out).repr() == array[0..8]` | Caller slices array before passing |
| `to_coefficient_array` | `out[0..8] == self.repr()` | Caller manages offset |

### Group 2 — Arithmetic (existing posts are correct)

| Method | Post | Status |
|---|---|---|
| `add` | `add_post`: `future[i] == lhs[i] + rhs[i]` + bound | Already correct — no change |
| `subtract` | `sub_post`: same | Already correct — no change |

### Group 3 — Modular arithmetic (upgrade to `mod_q_eq`)

New `mod_q_eq` definition (add to `specs.rs` F* block):
```fstar
let mod_q_eq (a b: int) : prop = a % 8380417 == b % 8380417
[@@ "opaque_to_smt"]
let mod_q_eq_opaque (a b: int) : prop = mod_q_eq a b
```

| Method | Current predicate | Upgraded predicate body |
|---|---|---|
| `montgomery_multiply` | `montgomery_multiply_lane_post`: raw `% 8380417` | `is_i32b 8380416 future[i] /\ mod_q_eq_opaque (v future[i]) (v lhs[i] * v rhs[i] * 8265825)` |
| `shift_left_then_reduce` | `shift_left_then_reduce_lane_post`: raw `% 8380417` | `is_i32b 8380416 future[i] /\ mod_q_eq_opaque (v future[i]) (v input[i] * 8192)` |
| `reduce` | `reduce_lane_post`: raw `% 8380417` | `is_i32b 8380416 future[i] /\ mod_q_eq_opaque (v future[i]) (v input[i])` |
| `infinity_norm_exceeds` | `infinity_norm_exceeds_post`: no `%` | No change needed |

**Note**: updating these predicates also updates the impl proofs that cite them. AVX2
`montgomery_multiply` body is already verified — upgrade it carefully with `--split_queries always`.

### Group 4 — Rejection sampling (predicates exist, cite Hacspec; needs wiring)

Predicates in `specs.rs` already cite Hacspec correctly. Drive-by: add to trait `ensures`.

| Method | Predicate (specs.rs) | Hacspec function |
|---|---|---|
| `rejection_sample_less_than_field_modulus` | `rejection_sample_3byte_lane_post` | `Hacspec_ml_dsa.Encoding.coeff_from_three_bytes` |
| `rejection_sample_less_than_eta_equals_2` | `rejection_sample_halfbyte_lane_post` (eta=2) | `Hacspec_ml_dsa.Encoding.coeff_from_half_byte` |
| `rejection_sample_less_than_eta_equals_4` | `rejection_sample_halfbyte_lane_post` (eta=4) | `Hacspec_ml_dsa.Encoding.coeff_from_half_byte` |

Proposed ensures shape (mirroring ML-KEM `compress_1_post`):
```rust
#[hax_lib::ensures(|_| fstar!(r#"
  v ${sampled_future} <= 256 /\
  Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
    rejection_sample_3byte_lane_post
      (Seq.index bytes0 i) (Seq.index bytes1 i) (Seq.index bytes2 i)
      (Seq.index coefficients_future i))
"#))]
```

### Group 5 — Serialization (BitVecEq generic-over-N, week 4 pre-work)

New predicates (week 4, after verifying BitVecEq on ML-DSA F* include path):

```fstar
(* Generic simple_bit_pack: coefficients in [0, 2^b), packed LSB-first.
   Mirrors ML-KEM serialize_post_N. *)
let simple_bit_pack_post_N (#n: usize) (b: nat{b > 0 /\ b <= 13})
    (input: t_Array i32 n)
    (output: t_Slice u8 {Seq.length output * 8 == v n * b}) : prop =
  BitVecEq.int_t_array_bitwise_eq input b output 8

(* Generic bit_pack: offset encoding, coefficients in [-a, b],
   stored as (b - x) per FIPS 204 Algorithm 17. *)
let bit_pack_post_N (#n: usize) (a b: nat) (w: nat{w > 0 /\ w <= 20})
    (input: t_Array i32 n)
    (output: t_Slice u8 {Seq.length output * 8 == v n * w}) : prop =
  let shifted = Seq.map (fun x -> mk_i32 (b - v x)) input in
  BitVecEq.int_t_array_bitwise_eq shifted w output 8
```

| Method | Bit width per coeff | `n` at trait level | Predicate |
|---|---|---|---|
| `t1_serialize` | 10 | 8 | `simple_bit_pack_post_N 10` |
| `t1_deserialize` | 10 | 8 | `simple_bit_unpack_post_N 10` |
| `t0_serialize` | 13, offset a=2^12-1 | 8 | `bit_pack_post_N (2^12-1) (2^12) 13` |
| `t0_deserialize` | 13 | 8 | `bit_unpack_post_N (2^12-1) (2^12) 13` |
| `commitment_serialize` | 5 or 6 (γ2-param) | 8 | `simple_bit_pack_post_N (bits_for_gamma2)` |
| `gamma1_serialize` | 18 or 20 (γ1-param) | 8 | `bit_pack_post_N γ1 γ1 w` |
| `gamma1_deserialize` | 18 or 20 | 8 | `bit_unpack_post_N γ1 γ1 w` |
| `error_serialize` | 3 or 4 (η-param) | 8 | `bit_pack_post_N η η w` |
| `error_deserialize` | 3 or 4 | 8 | `bit_unpack_post_N η η w` |

**Open**: `commitment_serialize` — is the bit width (5 or 6) a const generic or a runtime param?
Check `src/simd/avx2/encoding/commitment.rs` before implementing.

### Group 6 — NTT (new whole-ring posts, week 2-3)

New infrastructure in `specs.rs` F* block:
```fstar
(* Flatten 32 × 8-lane SIMD units into a 256-element polynomial. *)
let to_spec_poly (simd_units: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : t_Array i32 (mk_usize 256) =
  Seq.init (mk_usize 256) (fun k ->
    Seq.index (Seq.index simd_units (k / 8)) (k % 8))

[@@ "opaque_to_smt"]
let ntt_ring_post
    (input future: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32)) : prop =
  Hacspec_ml_dsa.Ntt.ntt (to_spec_poly input) == to_spec_poly future

[@@ "opaque_to_smt"]
let invert_ntt_ring_post
    (input future: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32)) : prop =
  Hacspec_ml_dsa.Ntt.intt (to_spec_poly input) == to_spec_poly future
```

Add to trait `ensures` clauses in `traits.rs`:
```rust
// ntt: add second ensures clause
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque (v ${specs::FIELD_MAX})
        (f_repr (Seq.index ${simd_units}_future i))) /\
    Libcrux_ml_dsa.Simd.Traits.Specs.ntt_ring_post
      (Seq.map f_repr ${simd_units})
      (Seq.map f_repr ${simd_units}_future)
"#))]
fn ntt(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);

// invert_ntt_montgomery: add second ensures clause
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (i:nat). i < 32 ==> Spec.Utils.is_i32b_array_opaque 4211177
        (f_repr (Seq.index ${simd_units}_future i))) /\
    Libcrux_ml_dsa.Simd.Traits.Specs.invert_ntt_ring_post
      (Seq.map f_repr ${simd_units})
      (Seq.map f_repr ${simd_units}_future)
"#))]
fn invert_ntt_montgomery(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]);
```

Proving the body admits: each impl (portable + AVX2) requires showing that the
layer-by-layer butterfly steps compose to the `Hacspec_ml_dsa.Ntt.ntt` equation.
Plan as 2-3 hr each. Use `reveal_opaque ntt_ring_post` + composition lemma.

---

## Sprint timeline

| Week | Task |
|------|------|
| Wk 2 | Add `mod_q_eq` + `to_spec_poly` + `ntt_ring_post` to specs.rs; wire Group 1/2/4 ensures; upgrade Group 3 predicates |
| Wk 2-3 | Prove portable + AVX2 `ntt` / `invert_ntt_montgomery` body admits using `ntt_ring_post` |
| Wk 4 | Verify BitVecEq on include path; add `bit_pack_post_N` / `simple_bit_pack_post_N`; wire Group 5 ensures |
| Wk 4-5 | Prove serialization body admits using BitVecEq; Hacspec bridge lemmas in `Commute.Chunk` |

---

## Open questions for week-2 finalization

1. **`mod_q_eq` placement**: define in `specs.rs` F* block (local) or `Spec.MLDSA.Math.fst`? Prefer local until shape is confirmed. Touching shared modules cascades 30+ min build.
2. **`BitVecEq` include path**: ML-KEM imports from `fstar-helpers/fstar-bitvec`. Confirm the same import is available in ML-DSA F* build before week-4 work begins.
3. **`commitment_serialize` bit-width**: check if it's a const generic (two separate methods) or a runtime param. Affects whether `simple_bit_pack_post_N` needs a `match on gamma2` wrapper.
4. **`to_spec_poly` scope**: private in `specs.rs` initially. Promote to shared spec module if `polynomial.rs` body proofs need it above the trait layer.
5. **Upgrading `montgomery_multiply_lane_post` to `mod_q_eq`**: the AVX2 `montgomery_multiply` body proof is currently verified. Upgrading the predicate body will require re-verifying. Confirm the team accepts this regression risk before week-2 starts.

# Pre/Post-chain audit for `generate_key_pair` panic-free sprint

**Scope:** read-only structural audit of `libcrux-ml-dsa` to inform a
multi-agent panic-free sprint that flips `generate_key_pair` /
`sign` / `verify` from `admit ()` body to
`#[hax_lib::fstar::verification_status(panic_free)]`.  Companion to
`proofs/handoff-2026-05-01-class-b-bounds.md`.

## Strict-polarity classification (per user mandate)

> Only **`verification_status(panic_free)` (with no `admit ()` body)**
> OR **full verification with NO admit/lax of any kind** counts as
> POSITIVE.  Anything else — `ADMIT_MODULES` Makefile entry, body
> `hax_lib::fstar!("admit ()")` (at any position), `verification_status(lax)`,
> per-fn `--admit_smt_queries true`, `hax_lib::opaque` — is NEGATIVE.
>
> A body admit at the **last** statement could in principle be
> equivalent to `panic_free` (admits the postcondition only),
> but it bypasses the canonical attribute mechanism and is therefore
> labeled lax/NEGATIVE — the marker is fishy.  In this codebase,
> empirically every body admit is at the **start** (line 1 of body),
> the worst-case form that admits the entire body, so the edge case
> doesn't currently apply.
>
> Note (per `~/.claude/projects/-Users-karthik-libcrux/memory`):
> `opaque_to_smt` is DIFFERENT and IS POSITIVE — it is an
> SMT-visibility hint, not a no-proof attribute; the function still
> verifies fully.

### File-level negative-marker sweep

```
$ grep -lE 'verification_status\(panic_free\)' src/**/*.rs
arithmetic.rs, encoding/error.rs, encoding/signature.rs,
encoding/signing_key.rs, encoding/t0.rs, encoding/verification_key.rs

$ grep -lE 'fstar!\("admit \(\)"\)' src/**/*.rs
arithmetic.rs, encoding/signature.rs, matrix.rs, ml_dsa_generic.rs,
sample.rs, simd/avx2.rs, simd/portable.rs

$ grep -c 'fstar!("admit ()")' src/simd/avx2.rs src/simd/portable.rs
src/simd/avx2.rs: 7
src/simd/portable.rs: 5
```

The original audit body undercounted body-admits by 12 (the
`simd/{avx2,portable}.rs` impl modules).  Total body-admits in the
stack: **32**, not 20.

### Strict-polarity per-module function tally

For each module in the stack: a function counts POSITIVE only if it
has *neither* a body `admit ()` *nor* lives in an `ADMIT_MODULES`
F* file *nor* carries `lax` / `opaque` / `--admit_smt_queries true`.
A function with `panic_free` AND a body admit is NEGATIVE — the body
admit defeats the panic_free check (e.g. `arithmetic::power2round_vector`,
`arithmetic::use_hint`, `encoding::signature::serialize`).

| Module | F* status | Total fns | POSITIVE | NEGATIVE | Notes |
|---|---|---:|---:|---:|---|
| `simd/traits.rs` | CHECK | 18 trait methods | 18 | 0 | Trait declarations; verified at the F* module level |
| `simd/portable.rs` (impls) | CHECK | many | many−5 | 5 | 5 body-admits |
| `simd/avx2.rs` (impls) | CHECK | many | many−7 | 7 | 7 body-admits |
| `simd/neon.rs` (impls) | CHECK | many | all | 0 | No body-admits |
| `simd/avx2/rejection_sample/shuffle_table.rs` | **ADMIT_MODULES** | 1 | 0 | 1 | Whole module admitted |
| `polynomial.rs` | CHECK | 8 | 8 | 0 | Fully verified |
| `arithmetic.rs` | CHECK | 7 | 5 | 2 | `power2round_vector`, `use_hint` (panic_free + body admit) |
| `ntt.rs` | CHECK | 4 | 4 | 0 | All verified |
| `sample.rs` | CHECK | 13 | 8 | 5 | 5 body-admits (`sample_up_to_four_*`, `sample_four_error_*`, `sample_mask_ring_element`, `sample_mask_vector`, `sample_challenge_ring_element`) |
| `samplex4.rs` | **ADMIT_MODULES** (×4 dispatchers) | 1 free fn + 1 trait + 3 impls = 5 | 0 | 5 | All four extracted F* files in `ADMIT_MODULES` |
| `matrix.rs` | CHECK | 6 | 3 | 3 | 3 body-admits (`compute_as1_plus_s2`, `compute_matrix_x_mask`, `compute_w_approx`) |
| `encoding/commitment.rs` | CHECK | 2 | 2 | 0 | Both verified |
| `encoding/error.rs` | CHECK | 3 | 3 | 0 | One panic_free, no admits |
| `encoding/gamma1.rs` | CHECK | 2 | 2 | 0 | Both verified |
| `encoding/signature.rs` | CHECK | 5 | 4 | 1 | `serialize` (panic_free + body admit) |
| `encoding/signing_key.rs` | CHECK | 1 | 1 | 0 | panic_free, no body admit |
| `encoding/t0.rs` | CHECK | 3 | 3 | 0 | One panic_free |
| `encoding/t1.rs` | CHECK | 2 | 2 | 0 | Both verified |
| `encoding/verification_key.rs` | CHECK | 2 | 2 | 0 | Both panic_free |
| `hash_functions.rs` (traits) | CHECK | 14 trait methods | 14 | 0 | All `requires(true)` (Class A) |
| `hash_functions.rs` (impls portable/simd256/neon) | CHECK | ~12 | 0 | ~12 | All structs `hax_lib::opaque` ⇒ NEGATIVE |
| `ml_dsa_generic.rs` | CHECK | 9 + 1 helper | 0 | 10 | Every public fn body-admitted |
| **TOTAL — modules in keygen cone** | — | ~115+ | ~80 | ~52 | |

**Positivity rate (keygen cone): ~60%.**  The remaining 40% breaks
down into roughly:
- 10 functions in `ml_dsa_generic.rs` (the targets)
- 5 `samplex4` dispatchers (in `ADMIT_MODULES`)
- 5 `sample.rs` body-admits
- 3 `matrix.rs` body-admits
- 2 `arithmetic.rs` panic_free-with-body-admit
- 1 `encoding/signature.rs` panic_free-with-body-admit
- 12 `simd/{avx2,portable}.rs` body-admits in SIMD-vector primitives
- 12 hash_functions impls (all `hax_lib::opaque` structs)
- 1 `simd/avx2/rejection_sample/shuffle_table.rs` (ADMIT_MODULES)

### Closure plan for `generate_key_pair`

To panic-free **just** `generate_key_pair`, the cone of NEGATIVES
that must flip POSITIVE (or have their ensures surfaced through the
ADMIT module barrier) is:

| File | NEGATIVE function | Closure needed |
|---|---|---|
| `ml_dsa_generic.rs` | `generate_key_pair` | The flip itself (panic_free + body admit removed). |
| `arithmetic.rs` | `power2round_vector` | Body proof — single inner-loop call to a fully-spec'd helper. |
| `matrix.rs` | `compute_as1_plus_s2` | Body proof — has the post-mismatch flagged below. |
| `sample.rs` | `sample_four_error_ring_elements`, `sample_up_to_four_ring_elements_flat` | NOT body-proven; only need *function-level ensures* surfaced for the call site to chain.  Body stays admitted (out-of-scope). |
| `samplex4.rs` | `sample_s1_and_s2`, `X4Sampler::matrix_flat` | Same — function-level ensures only; body-proof is gated by the `ADMIT_MODULES` decision (out-of-scope). |
| `simd/{portable,avx2}.rs` | 12 body-admits | NOT directly reached by `generate_key_pair`'s F* error chain — the SIMD primitives the panic-free flip uses (NTT, decompose, add, etc.) all have non-admit bodies in `simd/portable.rs`.  Audit-flag: confirm during sprint. |
| `encoding/{verification_key,signing_key}.rs` | none — already POSITIVE | Pre's match what Chain 1 + Chain 3 deliver. |

So the panic-free closure for `generate_key_pair` is:
**2 body-proofs** (`compute_as1_plus_s2`, `power2round_vector`) +
**~3 ensures-only surfacings** (`sample_*` and `samplex4::*`) +
**1 flip** (`generate_key_pair` itself).  ~5 work items, 1–3
sessions parallelized.

The same closure for `sign` and `verify` will pull in
`compute_matrix_x_mask` (1 more body-proof), `compute_w_approx`
(1 more body-proof), `encoding::signature::serialize` (1 more body-proof —
the panic_free flag is on but admit needs to come off), 4 more
sample helpers, and the `decompose_vector` / `make_hint` /
`use_hint` inner loop.  Roughly 2x the keygen scope.

Source tree: `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/src/`
F* outputs: `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa/proofs/fstar/extraction/`
Spec source: `/Users/karthik/libcrux-ml-dsa-proofs/specs/ml-dsa/src/`

`ADMIT_MODULES` (Makefile, lines 17–48): only the user-API wrappers
`Libcrux_ml_dsa.Ml_dsa_44/65/87.*`, the four `Samplex4.*`
dispatchers, and `Simd.Avx2.Rejection_sample.Shuffle_table` are
admitted.  Every other module in this stack is fully verified by SMT.
Body-level `hax_lib::fstar!("admit ()")` statements remain **inside
verified modules** (i.e. the function body is admitted but the
function's `requires`/`ensures` are still part of the module
interface).

---

## Summary table

| Module | F* module | F* status | # fns (non-test) | panic_free | body-admitted | with `requires` (real) | with `ensures` | spec-paired |
|---|---|---|---|---|---|---|---|---|
| `simd/traits.rs` | `Simd.Traits` | CHECK | 18 trait methods | n/a (trait) | n/a | 17 real + 1 `requires(true)` (`Repr::repr`) | 18 (lane/forall8 posts) | indirect (per-lane Spec.Utils) |
| `polynomial.rs` | `Polynomial` | CHECK | 8 (incl. 2 spec helpers) | 0 | 0 | 6 | 5 (lane-bound) | indirect (helpers) |
| `arithmetic.rs` | `Arithmetic` | CHECK | 6 + 1 helper | 2 (`power2round_vector`, `use_hint`) | 2 (`power2round_vector`, `use_hint`) | 6 | 4 | `power2round`, `use_hint`, `make_hint`, `decompose` |
| `ntt.rs` | `Ntt` | CHECK | 4 | 0 | 0 | 4 | 4 | `ntt`, `intt` (and helpers) |
| `sample.rs` | `Sample` | CHECK | 12 (some `pub(crate)`) | 0 | **5 body-admits** | 6 | 5 (length / counter bounds) | partial (`expand_a` / `expand_s` / `expand_mask`, `sample_in_ball`) |
| `samplex4.rs` | `Samplex4` (+ Avx2/Neon/Portable) | **ADMIT** (subdir) | 1 free fn + 1 trait + 3 impls | 0 | 0 (relies on ADMIT) | trait method `requires(true)` | trait + free fn length-only | `expand_a` (matrix), `expand_s` (s1_s2) |
| `matrix.rs` | `Matrix` | CHECK | 6 | 0 | **3 body-admits** (`compute_as1_plus_s2`, `compute_matrix_x_mask`, `compute_w_approx`) | 6 | 6 (length + bounds) | `matrix_vector_ntt`, vector ops |
| `encoding/commitment.rs` | `Encoding.Commitment` | CHECK | 2 | 0 | 0 | 2 | 2 (length-only) | `simple_bit_pack` (w1_encode) |
| `encoding/error.rs` | `Encoding.Error` | CHECK | 3 | 1 (`deserialize_to_vector_then_ntt`) | 0 | 3 | 3 | `bit_unpack(..,η,η)` (sk_decode), composed with `ntt` |
| `encoding/gamma1.rs` | `Encoding.Gamma1` | CHECK | 2 | 0 | 0 | 2 | 2 | `bit_pack(..,γ1-1,γ1)` / `bit_unpack(..,γ1-1,γ1)` |
| `encoding/signature.rs` | `Encoding.Signature` | CHECK | 4 (+1 dummy + `set_hint`) | 4 (`serialize`, `deserialize`, `validate_hint_rows`, `write_hint_rows`) | **1** (`serialize`) | 5 | 0 | `sig_encode` / `sig_decode` (+ `hint_bit_pack` / `hint_bit_unpack`) |
| `encoding/signing_key.rs` | `Encoding.Signing_key` | CHECK | 1 | 1 (`generate_serialized`) | 0 (only ext. `assume`) | 1 | 0 | `sk_encode` |
| `encoding/t0.rs` | `Encoding.T0` | CHECK | 3 | 1 (`deserialize_to_vector_then_ntt`) | 0 | 3 | 3 | `bit_pack(2^d-1,2^d)` / `bit_unpack` |
| `encoding/t1.rs` | `Encoding.T1` | CHECK | 2 | 0 | 0 | 2 | 2 | `simple_bit_pack(2^10)` / `simple_bit_unpack` |
| `encoding/verification_key.rs` | `Encoding.Verification_key` | CHECK | 2 | 2 (`generate_serialized`, `deserialize`) | 0 | 2 | 2 | `pk_encode` / `pk_decode` |
| `hash_functions.rs` | `Hash_functions.{Shake128,Shake256,Portable,Simd256,Neon}` | CHECK | 4 traits, 14 trait methods, 12+ impls | 0 (impls have no annotations; bodies are pass-through) | 0 | 14 trait methods all `requires(true)` | 11 trait methods (length) | n/a (opaque crypto) |
| `ml_dsa_generic.rs` | `Ml_dsa_generic` (+ per-set sub-modules) | CHECK on top, `Ml_dsa_44/65/87.*` ADMIT | 9 (`generate_key_pair`, `sign{_internal,_mut,}`, `verify{_internal,}`, `sign_pre_hashed{,_mut}`, `verify_pre_hashed`, `derive_message_representative`) | 0 | **9 body-admits** (every public function) | 0 (only `cfg_attr(hax)`-gated `requires`) | 3 spec-equality (`generate_key_pair`, `sign`, `verify`) | yes — `keygen_internal`, `sign`, `verify` |
| **Totals** | | 14 CHECK / 4 ADMIT | ~85 fns | **11 panic_free** | **20 body-admits** | broad coverage | broad coverage | top-level wired |

Notes on counts above:
- Trait methods listed once per definition (default impls are inherited).
- `derive_message_representative` is private but body-admitted; counted under `ml_dsa_generic.rs`.
- `Samplex4` admit comes from the **module being in `ADMIT_MODULES`**, not per-fn body admits.
- "with `requires`" excludes pure `#[requires(true)]` opacity scaffolding (counted separately).

---

## Module-by-module detail

### `simd/traits.rs` (`Libcrux_ml_dsa.Simd.Traits.fst`)

CHECK. 18 trait methods, all carry real `requires` (Spec.Utils
predicates, `is_i32b_array_opaque`, length, eta/gamma2 case constants).
Posts use `is_i32b_array_opaque`/`is_i32b_strict_lower_array_opaque`
or `forall8 lane_post` shapes.

| Method | requires | ensures shape |
|---|---|---|
| `zero` | `requires(true)` | `repr() == [0; 8]` |
| `from_coefficient_array`/`to_coefficient_array` | length=8 | length-preserving + repr eq |
| `add` / `subtract` | `add_pre`/`sub_pre` | `add_post`/`sub_post` |
| `infinity_norm_exceeds` | bound>0, `is_i32b_array_opaque FIELD_MAX` | `infinity_norm_exceeds_post` |
| `decompose` | gamma2 ∈ {V261888, V95232}, FIELD_MAX bound | dual γ2-conditional bound + `decompose_lane_post` |
| `compute_hint` | gamma2 case + FIELD_MAX bounds on low/high | hint binary bound + `compute_hint_lane_post` |
| `use_hint` | gamma2 case + FIELD_MAX bound, hint binary | dual γ2-conditional `is_i32b 16/44` |
| `montgomery_multiply` | rhs FIELD_MAX | lhs FIELD_MAX + lane_post |
| `shift_left_then_reduce` | SHIFT_BY=13, lane in [0, 261631] | `shift_left_then_reduce_lane_post` |
| `power2round` | t0 FIELD_MAX | t0 strict `pow2 12`, t1 ∈ [0, pow2 10) + lane post |
| `rejection_sample_less_than_field_modulus` | length ratios | result≤8, length pres., per-coeff [0,Q) |
| `rejection_sample_less_than_eta_equals_{2,4}` | length ratios | result≤8, [-η,η] |
| `gamma1_serialize`/`commitment_serialize`/`error_serialize`/`t0_serialize`/`t1_serialize` | length + `is_pos_array_opaque` (or strict_lower) | length-preserving |
| `gamma1_deserialize`/`error_deserialize`/`t0_deserialize`/`t1_deserialize` | length | per-lane bound |
| `ntt` | `is_i32b NTT_BASE_BOUND` per simd_unit | `is_i32b FIELD_MAX` |
| `invert_ntt_montgomery` | `is_i32b FIELD_MAX` | `is_i32b FIELD_MAX` |
| `reduce` | `is_i32b 2143289343` | `is_i32b FIELD_MAX` + lane_post |

Plus `Repr::repr` (super-trait): `requires(true)` only.

The trait surface is **the most polished pre/post layer in the
crate**.  Every method has the right precondition for its callers.
No work needed here for the panic-free sprint.

### `polynomial.rs` (`Libcrux_ml_dsa.Polynomial.fst`)

CHECK. Generic `PolynomialRingElement<SIMDUnit>` wrappers around the
trait.  Two spec helpers (`is_bounded_simd_unit`, `is_bounded_poly`)
defined in `pub(crate) mod spec` — `is_bounded_poly` is `opaque_to_smt`
with three `lemma_*` lookups attached via `fstar::after`.

| Function | `requires` | `ensures` |
|---|---|---|
| `zero` | none | `is_i32b_array_opaque 0` per simd_unit |
| `to_i32_array` | none | none |
| `from_i32_array` | `array.len() == 256` | none |
| `infinity_norm_exceeds` | bound>0 + per-i FIELD_MAX bound | none |
| `add` | per-i `add_pre` | per-j `add_post` |
| `add_bounded` (b1,b2) | `b1+b2 ≤ i32::MAX` + per-j bounds | `is_i32b (b1+b2)` per j |
| `subtract` | per-i `sub_pre` | per-j `sub_post` |
| `subtract_bounded` (b1,b2) | symmetric to `add_bounded` | symmetric |

No body-admits, no panic_free flips.  All loops carry
`loop_invariant!` predicates that mirror the post.  This module is
ready as-is.

### `arithmetic.rs` (`Libcrux_ml_dsa.Arithmetic.fst`)

CHECK.

| Function | panic_free | body-admit | `requires` | `ensures` |
|---|---|---|---|---|
| `vector_infinity_norm_exceeds` | no (opaque_to_smt) | no | bound>0 + FIELD_MAX per i,j | none |
| `shift_left_then_reduce` | no (opaque_to_smt) | no | SHIFT_BY=13, [0,261631] per i,j | per-i `is_i32b FIELD_MAX` |
| `power2round_vector` | **YES** | **YES** (`admit ()`) | t0.len==t1.len + per-i FIELD_MAX | per-i strict `pow2 12` + per-j `[0, pow2 10)` |
| `power2round_one_ring_element` (private) | no | no | per-j FIELD_MAX | per-j strict `pow2 12` + `[0, pow2 10)` |
| `decompose_vector` | no (opaque_to_smt) | no | γ2 case + length deps + per-i,j FIELD_MAX | none (declared internal) |
| `make_hint` | no (opaque_to_smt, rlimit 200) | no | γ2 case + length deps + per-i,j FIELD_MAX | none |
| `use_hint` | **YES** | **YES** | γ2 case + lengths + hint binary + per-i,j FIELD_MAX | per-i,j `is_i32b 16/44` (γ2-conditional) |

Two body admits. `power2round_vector` body has only one inner-loop
call to a fully-spec'd helper; the body-admit could be discharged in
~1 session.  `use_hint` similarly straightforward.  Both are needed
for `generate_key_pair` Chain 3.

### `ntt.rs` (`Libcrux_ml_dsa.Ntt.fst`)

CHECK.

| Function | requires | ensures |
|---|---|---|
| `ntt` | per-i `is_i32b NTT_BASE_BOUND` | per-i `is_i32b FIELD_MAX` |
| `invert_ntt_montgomery` | per-i `is_i32b FIELD_MAX` | per-i `is_i32b FIELD_MAX` |
| `reduce` | per-i `is_i32b 2143289343` | per-i `is_i32b FIELD_MAX` + lane reduce_post |
| `ntt_multiply_montgomery` | per-i FIELD_MAX on rhs | per-i FIELD_MAX on lhs + lane montgomery_multiply_post |

No body admits, all chain.  This module is ready.

**Critical:** `NTT_BASE_BOUND` is currently set so that `pow2 12 ≤
NTT_BASE_BOUND` and `11 ≤ NTT_BASE_BOUND` (verified by the lifts in
`encoding::t0::deserialize_to_vector_then_ntt` and
`encoding::error::deserialize_to_vector_then_ntt`).  Per the
ml-dsa-proofs commit `686543e33` referenced in `matrix.rs` line 297,
NTT_BASE_BOUND was widened to `FIELD_MAX`.

### `sample.rs` (`Libcrux_ml_dsa.Sample.fst`)

CHECK at the F* module level, but **5 body-admits**.

| Function | panic_free | body-admit | `requires` | `ensures` |
|---|---|---|---|---|
| `rejection_sample_less_than_field_modulus` (private) | no | no | `*sampled_coefficients < 256` | `*sampled_coefficients ≤ 263` |
| `generate_domain_separator` (private) | no | no | none | none |
| `add_domain_separator` | no | no | `slice.len() ≤ 32` | none |
| `sample_up_to_four_ring_elements_flat` | no | **YES** | none (only nested fn `xy` has `width != 0`) | none |
| `rejection_sample_less_than_eta_equals_2/4` (private) | no | no | counter < 256 | counter ≤ 263 |
| `rejection_sample_less_than_eta` | no | no | counter < 256 | counter ≤ 263 |
| `add_error_domain_separator` | no | no | `slice.len() ≤ 64` | none |
| `sample_four_error_ring_elements` | no | **YES** | none | none |
| `sample_mask_ring_element` (private) | no | **YES** | none | none |
| `sample_mask_vector` | no | **YES** | none | none |
| `inside_out_shuffle` (private) | no | no | `*out_index < 256` | `*out_index ≤ 256` |
| `sample_challenge_ring_element` | no | **YES** | none | none |

The four body-admits at this level are the bulk of the panic-free
deficit on the sample side.  None of them currently expose a bound
ensures.  All four are reached from `generate_key_pair` (via
`samplex4::sample_s1_and_s2`) or from `sign_internal` /
`verify_internal`.  Each needs:
1. A coefficient-bound ensures (`forall i j. abs(re[i].coefs[j]) ≤ η`
   for `sample_four_error_ring_elements` /
   `sample_s1_and_s2`; `< pow2 γ1_exp` for `sample_mask_ring_element` /
   `sample_mask_vector`; binary {-1,0,1} for
   `sample_challenge_ring_element`).
2. Body proof of panic-freedom — currently `admit ()`.

### `samplex4.rs` (`Libcrux_ml_dsa.Samplex4.fst` + 3 impl modules)

**ADMIT** for all four extraction modules (`Samplex4.fst`,
`Samplex4.{Avx2,Neon,Portable}.fst`).

| Item | Annotations |
|---|---|
| `X4Sampler` trait | `#[hax_lib::attributes]` only |
| `X4Sampler::matrix_flat` (trait method) | `requires(true)`, `ensures(future(matrix).len() == matrix.len())` (length-preserving) |
| `matrix_flat` (free fn) | none |
| `sample_s1_and_s2` (free fn) | `ensures(future(s1_s2).len() == s1_s2.len())` (length-preserving only) |
| `PortableSampler::matrix_flat` impl | none (delegates to `matrix_flat`) |
| `NeonSampler::matrix_flat` impl | none |
| `AVX2Sampler::matrix_flat` impl | unsafe target_feature wrapper, no annotations |

Class A landed: `requires(true)` on the trait method, length-only
ensures on `sample_s1_and_s2` and the trait method.  Notably
**neither has a coefficient-bound ensures** — that's the Class B
sprint's task.

The fact that `Samplex4.*.fst` are all in `ADMIT_MODULES` is a
**deliberate scoping decision** (Session D in
`proofs/post-merge-handoff.md`) — they admit because the underlying
X4 Xof hash trait method panic-freedoms aren't yet established.
Class B does not need to remove them from `ADMIT_MODULES`; the
length-preserving + coefficient-bound `ensures` can be added at the
function level even while the body remains admitted via the
ADMIT_MODULES route.  (The wider issue: the F* file is module-level
admitted, so the in-file `ensures` is treated as a postulated
property — exactly what we want for the panic_free sprint.)

### `matrix.rs` (`Libcrux_ml_dsa.Matrix.fst`)

CHECK at module level, **3 body-admits**.

| Function | panic_free | body-admit | `requires` (summary) | `ensures` |
|---|---|---|---|---|
| `compute_as1_plus_s2` | no | **YES** | length deps, `columns_in_a ≤ 7`, FIELD_MAX on s1_ntt, s1_s2, result | length-preserving + per-i,j `is_i32b 16760832` (≈2*FIELD_MAX) |
| `compute_matrix_x_mask` | no | **YES** | length deps, `columns_in_a ≤ 7`, FIELD_MAX on matrix, result | length-pres. + per-i,j FIELD_MAX |
| `vector_times_ring_element` | no (rlimit 800 split) | no | per-i,j FIELD_MAX on vec + ring_element | length-pres. + per-i,j FIELD_MAX |
| `add_vectors` | no (rlimit 800 split) | no | per-i,j FIELD_MAX on lhs, rhs | length-pres. + per-i,j `is_i32b 16760832` |
| `subtract_vectors` | no (rlimit 800 split) | no | symmetric | symmetric |
| `compute_w_approx` | no | **YES** | length deps + FIELD_MAX on matrix/sig_response + verifier_challenge + t1 in [0,261631] | length-preserving only |

`compute_as1_plus_s2`'s body-admit is the central blocker for Class B
Chain 3; its `ensures` is fully specified (the spec contract is in
place), so the sprint just needs to discharge the body proof — the
post is an upper bound from `columns_in_a` additions of FIELD_MAX
elements (≤ 7·FIELD_MAX < 2^25 < 16760832 = 2·FIELD_MAX), plus the
final `+ s1_s2[..]` and Barrett `reduce` capping to FIELD_MAX-ish.

`compute_matrix_x_mask` and `compute_w_approx` shape-match
`compute_as1_plus_s2` and `verify`'s consumption — known-tractable
with the same recipe.

### `encoding/commitment.rs` (`Libcrux_ml_dsa.Encoding.Commitment.fst`)

CHECK. Both fns clean.

| Function | requires | ensures |
|---|---|---|
| `serialize` (private) | length 128/192, per-j `is_pos_array_opaque (pow2 d - 1)` | length-pres. |
| `serialize_vector` | length deps + per-k,j is_pos bound | length-pres. |

No work needed for keygen.  Used in sign/verify.

### `encoding/error.rs` (`Libcrux_ml_dsa.Encoding.Error.fst`)

CHECK.

| Function | panic_free | requires | ensures |
|---|---|---|---|
| `serialize` | no | length + per-j `is_pos_array_opaque eta` | length-pres. |
| `chunk_size` (private) | no | none | none |
| `deserialize` (private) | no | length | per-j `is_i32b 11` (lift of η-conditional `forall8`) |
| `deserialize_to_vector_then_ntt` | **YES** | length deps | length-pres. |

`deserialize_to_vector_then_ntt` is panic_free, with full chain to
`ntt`'s pre via `is_i32b_array_larger 11 NTT_BASE_BOUND` lemma.
Recipe used as a template for `t0`'s analogue.  Used by
`sign_internal`, not keygen.

### `encoding/gamma1.rs` (`Libcrux_ml_dsa.Encoding.Gamma1.fst`)

CHECK.  Both fns clean.

| Function | requires | ensures |
|---|---|---|
| `serialize` | γ1_exp ∈ {17,19} + length + `is_pos pow2 γ1_exp - 1` per j | length-pres. |
| `deserialize` | γ1_exp ∈ {17,19} + length | per-j `is_i32b (pow2 γ1_exp)` |

No work needed for keygen.  Used in sign/verify.

### `encoding/signature.rs` (`Libcrux_ml_dsa.Encoding.Signature.fst`)

CHECK.

| Function | panic_free | body-admit | requires | ensures |
|---|---|---|---|---|
| `serialize` | **YES** | **YES** | γ1_exp case + length + hint dimensions + `count_total_ones hint ≤ ω` + ω+rows ≤ usize | none |
| `set_hint` (private) | no | no | i < out_hint.len() ∧ j < 256 | none |
| `deserialize` | **YES** | no | γ1_exp case + length deps | none |
| `validate_hint_rows` (private) | **YES** | no | length=ω+rows | none (returns tuple) |
| `write_hint_rows` (private) | **YES** | no | length=ω+rows + out_hint.len()=rows | none |

`signature::serialize` body-admit is the only deficit in this module
— the panic_free flip is on but the body has `admit ()` because of
the two outstanding obligations documented in the source comment
(`gamma1::serialize` per-element bound, and `count_total_ones`-based
inner-loop bound).  Out of scope for keygen but central for sign.

### `encoding/signing_key.rs` (`Libcrux_ml_dsa.Encoding.Signing_key.fst`)

CHECK.  One function only, fully wired.

| Function | panic_free | body-admit | requires | ensures |
|---|---|---|---|---|
| `generate_serialized` | **YES** | no (uses `assume` for SHA3 trait) | seed lengths + s1_2/t0 dimensions + per-k,j `is_pos eta` (s1_2) and `is_i32b strict_lower (pow2 12)` (t0) | none |

Already panic_free.  Pre exactly matches what Chain 1 + Chain 3 need
to deliver.  This is **the consumption end of both Chain 1 and Chain 3
for keygen.**

### `encoding/t0.rs` (`Libcrux_ml_dsa.Encoding.T0.fst`)

CHECK.

| Function | panic_free | requires | ensures |
|---|---|---|---|
| `serialize` | no | length + per-j strict_lower `pow2 12` | length-pres. |
| `deserialize` (private) | no | length | per-j strict_lower `pow2 12` |
| `deserialize_to_vector_then_ntt` | **YES** | length deps | none |

Pattern-template for `error::deserialize_to_vector_then_ntt`.  No
work needed for keygen.

### `encoding/t1.rs` (`Libcrux_ml_dsa.Encoding.T1.fst`)

CHECK.

| Function | requires | ensures |
|---|---|---|
| `serialize` | length=320 + per-j,i `0 ≤ v < pow2 10` | length-pres. |
| `deserialize` | length=320 | per-j,i `0 ≤ v < pow2 10` |

No work needed.  Direct upstream consumer is `verification_key::generate_serialized`.

### `encoding/verification_key.rs` (`Libcrux_ml_dsa.Encoding.Verification_key.fst`)

CHECK.

| Function | panic_free | requires | ensures |
|---|---|---|---|
| `generate_serialized` | **YES** | seed length + t1 dim + buf length + per-k,j,i `0 ≤ v < pow2 10` | length-pres. |
| `deserialize` | **YES** | rows ≤ 8 + length deps | per-k,j,i `0 ≤ v < pow2 10` |

Already panic_free.  Pre exactly matches what Chain 3
(`power2round_vector → t1`) delivers.

### `hash_functions.rs` (`Libcrux_ml_dsa.Hash_functions.*`)

CHECK.  4 traits (shake256::DsaXof, shake256::XofX4, shake256::Xof,
shake128::Xof, shake128::XofX4 — actually 5).  All trait methods
carry `#[requires(true)]` (Class A pattern) and length-preserving
`#[ensures(...)]` where applicable.  Impl modules (`portable`,
`simd256`, `neon`) carry `#[cfg_attr(hax, hax_lib::opaque)]` on
state structs and have **no per-fn annotations**; bodies are pass-
through to `libcrux_sha3::*`.

No work needed.

### `ml_dsa_generic.rs` (`Libcrux_ml_dsa.Ml_dsa_generic.*`)

CHECK at outer module, **9 body-admits**.

| Function | panic_free | body-admit | spec-paired ensures |
|---|---|---|---|
| `generate_key_pair` | no | **YES** | `keygen_internal` (full state equality) |
| `sign_internal` | no | **YES** | none |
| `verify_internal` | no | **YES** | none |
| `sign_pre_hashed_mut` | no | **YES** | none |
| `sign_pre_hashed` | no | **YES** | none |
| `sign_mut` | no | **YES** | none |
| `sign` | no | **YES** | `hacspec_ml_dsa::sign` (spec equality, conditional on input lengths) |
| `verify` | no | **YES** | `hacspec_ml_dsa::verify` (spec equality, conditional on input lengths) |
| `verify_pre_hashed` | no | **YES** | none |
| `derive_message_representative` | no | **YES** | none |

**Length precondition not yet present** on `generate_key_pair` (the
sprint plan documents it as test scaffolding to be re-applied when
flipping to panic_free).  `sign`/`verify`'s ensures use
`hax_lib::implies(...)` to bake the length conjunction in as a
guard.

---

## Class B chain refinement

### Chain 1 — NTT-bound chain

Helpers between `samplex4::sample_s1_and_s2` and `ntt::ntt` (for the
`s1_ntt` slice in `generate_key_pair`):

| Helper | Has length ensures? | Has bound ensures? | What's missing |
|---|---|---|---|
| `samplex4::sample_s1_and_s2` | YES (length only) | NO | Add `forall i j. is_i32b_array_opaque η (s1_s2[i].simd_units[j])` |
| `samplex4::X4Sampler::matrix_flat` (trait + 3 impls) | YES (length only) | NO (and not needed for keygen — output goes through `expand_a`'s reject-sample, so naturally `< Q < FIELD_MAX`) | Add `is_i32b_array_opaque FIELD_MAX` per coefficient — required for `compute_as1_plus_s2` pre |
| `sample::sample_four_error_ring_elements` | NO | NO (body admitted) | Add length + per-coeff η bound |
| `sample::sample_up_to_four_ring_elements_flat` | NO | NO (body admitted) | Length + per-coeff `is_i32b FIELD_MAX` |
| `sample::rejection_sample_less_than_eta` (private) | NO (counter only) | counter ≤ 263 | OK as-is (the sample fns above consume it) |
| `sample::rejection_sample_less_than_field_modulus` (private) | NO | counter only | OK as-is |
| `polynomial::PolynomialRingElement::from_i32_array` | NO | NO | Length: `array.len() == 256`; needs no bound ensures (carries the input via SIMD trait pre) |
| `ntt::ntt` | n/a | input pre `is_i32b NTT_BASE_BOUND`, output post FIELD_MAX | OK |

**Chain 1 net work:**
1. Surface a per-coeff η bound on `sample_four_error_ring_elements`
   and propagate to `sample_s1_and_s2`.  (Body-admit can stay; ensures
   is the contract.)
2. Surface FIELD_MAX coefficient bound on
   `samplex4::matrix_flat` (trait method + free fn).  (Required for
   `compute_as1_plus_s2`'s `a_as_ntt` pre, line 26 of `matrix.rs`'s
   `compute_as1_plus_s2`.)
3. **Bigger than the handoff estimated:** the bound on
   `sample_four_error_ring_elements` requires similar treatment on
   `sample_up_to_four_ring_elements_flat` and the `from_i32_array`
   bridge from the local `[i32; 263]` buffer into the
   `PolynomialRingElement`.  Each inner sampler — `_eta_2`, `_eta_4`,
   `_field_modulus` — currently has only counter ensures, not coeff
   bound.

### Chain 2 — encoding chain

End of `generate_key_pair`'s body:

| Helper | panic_free? | What's missing |
|---|---|---|
| `encoding::verification_key::generate_serialized` | YES | Pre: `forall k j i. 0 ≤ v < pow2 10` on `t1` — must be discharged from Chain 3's `power2round_vector` post.  Already shape-matches. |
| `encoding::t1::serialize` | no (but verifies) | Pre: per-j,i `0 ≤ v < pow2 10`.  Cleanly chains. |
| `encoding::signing_key::generate_serialized` | YES | Pre: `is_pos η` on `s1_2` (from Chain 1) **and** strict_lower `pow2 12` on `t0` (from Chain 3).  Both are sourced from upstream chains. |
| `encoding::error::serialize` | no (but verifies) | Pre: `is_pos η`.  Cleanly chains. |
| `encoding::t0::serialize` | no (but verifies) | Pre: strict_lower `pow2 12`.  Cleanly chains. |

**Chain 2 net work is light** — both `generate_serialized` calls are
already panic_free and have the right `requires` shape.  The chain
"closes" iff Chain 1 surfaces an `is_pos η` ensures on `s1_s2` and
Chain 3 surfaces a strict_lower `pow2 12` ensures on `t0`.

**However:** the precondition on `signing_key::generate_serialized`
asks for `is_pos_array_opaque η` (non-negative shifted form), while
the natural sample output is `is_i32b η` (centered).  The
`sample_s1_and_s2` ensures Chain 1 sets MUST match the consumer's
shape — likely needs to be `is_pos_array_opaque η` (since that's what
`error::serialize` and `signing_key::generate_serialized` consume),
or a bridge lemma at the call site.

### Chain 3 — arithmetic / matrix chain

| Helper | panic_free? | body-admit? | What's missing |
|---|---|---|---|
| `matrix::compute_as1_plus_s2` | no | YES | Body proof.  Pre: FIELD_MAX on a_as_ntt, s1_ntt, s1_s2, result.  Post: `is_i32b 16760832` (≈2·FIELD_MAX).  Needed by `power2round_vector` pre (FIELD_MAX). |
| `arithmetic::power2round_vector` | YES | YES | Body proof — single inner call.  Pre: per-i,j FIELD_MAX.  Post: t0 strict `pow2 12`, t1 ∈ [0,1024). |
| `arithmetic::power2round_one_ring_element` (private) | no | no | Already verified (loop_invariant is in place). |
| `ntt::ntt_multiply_montgomery` | no | no | Already verified.  Pre: rhs FIELD_MAX.  Post: lhs FIELD_MAX. |
| `ntt::reduce` | no | no | Already verified.  Pre: input ≤ 2143289343.  Post: FIELD_MAX. |
| `ntt::invert_ntt_montgomery` | no | no | Already verified.  Pre/post: FIELD_MAX. |

**Chain 3 net work:**
1. **`compute_as1_plus_s2` body proof.** Body has nested loops (i,j)
   plus a final pass for reduce + invert + add s1_s2.  The final post
   `is_i32b 16760832` is loose enough — `2·FIELD_MAX = 16760834`,
   `16760832 = 2·FIELD_MAX − 2` — that the bound is comfortable.  Needs
   inner `Polynomial::add_bounded`-style ghost bounds on each iteration.
2. **`power2round_vector` body proof.** Per-i call to a fully-spec'd
   `power2round_one_ring_element`.  Trivial loop_invariant chain.

Net: `compute_as1_plus_s2` is the dominant Chain 3 body-proof item.
A precedent for the pattern lives in `matrix::add_vectors` /
`subtract_vectors` (rlimit 800 + split_queries always +
`add_bounded` accumulator).

`compute_as1_plus_s2`'s output bound `is_i32b 16760832` only carries
through `power2round_vector`'s pre as **FIELD_MAX**.  The chain
discharges via `is_i32b_array_larger 16760832 FIELD_MAX` since
`16760832 < FIELD_MAX = 8380416`?  **No — this is wrong.**
`16760832 > FIELD_MAX`.  So the pre needs a **`reduce` step** between
`compute_as1_plus_s2` and `power2round_vector`, **or** the
`compute_as1_plus_s2` post needs to be tightened to FIELD_MAX (which
is what happens in the body, since the final op is `+ s1_s2[..]`
where each is in [-η,η], i.e. ≤ FIELD_MAX, but only after a
`reduce`).  **Audit-flag: the current `compute_as1_plus_s2` post
declares `16760832` but inspects show the body does call `reduce`
internally, so the post should be tightenable to FIELD_MAX.** This is
a documented gap in Chain 3.

---

## Spec-mapping table (forward-looking)

| Impl function | Spec function (`Hacspec_ml_dsa.*`) | Mapping confidence |
|---|---|---|
| `ml_dsa_generic::generate_key_pair` | `ml_dsa::keygen_internal` | HIGH (already wired in `ensures`) |
| `ml_dsa_generic::sign` | `ml_dsa::sign` | HIGH (already wired) |
| `ml_dsa_generic::verify` | `ml_dsa::verify` | HIGH (already wired) |
| `ml_dsa_generic::sign_internal` | `ml_dsa::sign_internal` | HIGH (no ensures wired yet, but spec exists) |
| `ml_dsa_generic::verify_internal` | `ml_dsa::verify_internal` | HIGH |
| `samplex4::matrix_flat` (and `X4Sampler::matrix_flat`) | `sampling::expand_a` | HIGH |
| `samplex4::sample_s1_and_s2` | `sampling::expand_s` | HIGH |
| `sample::sample_four_error_ring_elements` | a fragment of `sampling::expand_s` (4-at-a-time) | MEDIUM (impl-only chunking) |
| `sample::sample_up_to_four_ring_elements_flat` | a fragment of `sampling::expand_a` | MEDIUM |
| `sample::rejection_sample_less_than_field_modulus` (private) | `sampling::rej_ntt_poly` (per-poly) | MEDIUM |
| `sample::rejection_sample_less_than_eta` | `sampling::rej_bounded_poly` (per-poly) | MEDIUM |
| `sample::sample_mask_vector` | `sampling::expand_mask` | HIGH |
| `sample::sample_mask_ring_element` (private) | a per-element fragment of `sampling::expand_mask` | MEDIUM |
| `sample::sample_challenge_ring_element` | `sampling::sample_in_ball` | HIGH |
| `ntt::ntt` | `ntt::ntt` | HIGH |
| `ntt::invert_ntt_montgomery` | `ntt::intt` | HIGH (Montgomery domain conversion) |
| `ntt::reduce` | identity (centered modular) | LOW (impl helper, no direct spec) |
| `ntt::ntt_multiply_montgomery` | `polynomial::poly_pointwise_mul` (Montgomery domain) | MEDIUM |
| `matrix::compute_as1_plus_s2` | A fragment of `keygen_internal` (`A·s₁ + s₂`); uses `matrix::matrix_vector_ntt` + `polynomial::vector_intt` + `polynomial::vector_add` | MEDIUM |
| `matrix::compute_matrix_x_mask` | A fragment of `try_sign_iteration` (`A·y` line of FIPS 204 Alg 7) | MEDIUM |
| `matrix::compute_w_approx` | A fragment of `verify_internal` (`A·z − c·t₁·2ᵈ` line) | MEDIUM |
| `matrix::vector_times_ring_element` | `polynomial::scalar_vector_ntt` | HIGH |
| `matrix::add_vectors` | `polynomial::vector_add` | HIGH |
| `matrix::subtract_vectors` | `polynomial::vector_sub` | HIGH |
| `arithmetic::vector_infinity_norm_exceeds` | `polynomial::vector_infinity_norm` (predicate form vs scalar) | HIGH |
| `arithmetic::power2round_vector` | `polynomial::vector_power2round` | HIGH |
| `arithmetic::decompose_vector` | `polynomial::vector_high_bits` + `polynomial::vector_low_bits` | HIGH (2-tuple split) |
| `arithmetic::make_hint` | `polynomial::vector_make_hint` (with bool→bit cast + count) | HIGH |
| `arithmetic::use_hint` | `polynomial::vector_use_hint` | HIGH |
| `arithmetic::shift_left_then_reduce` | A fragment of `verify_internal` (`t₁·2ᵈ` step) | LOW (impl helper) |
| `encoding::verification_key::generate_serialized` | `encoding::pk_encode` | HIGH |
| `encoding::verification_key::deserialize` | `encoding::pk_decode` | HIGH |
| `encoding::signing_key::generate_serialized` | `encoding::sk_encode` | HIGH |
| `encoding::t0::serialize` / `deserialize` | `encoding::bit_pack(2^d-1, 2^d)` / `bit_unpack` | HIGH |
| `encoding::t0::deserialize_to_vector_then_ntt` | `bit_unpack ∘ ntt` (per-poly composition) | HIGH |
| `encoding::t1::serialize` / `deserialize` | `encoding::simple_bit_pack(2^10)` / `simple_bit_unpack` | HIGH |
| `encoding::error::serialize` / `deserialize` | `encoding::bit_pack(η, η)` / `bit_unpack` | HIGH |
| `encoding::error::deserialize_to_vector_then_ntt` | `bit_unpack(η,η) ∘ ntt` (per-poly composition) | HIGH |
| `encoding::commitment::serialize` / `serialize_vector` | `encoding::w1_encode` / `simple_bit_pack` (per-poly + fold) | HIGH |
| `encoding::gamma1::serialize` / `deserialize` | `encoding::bit_pack(γ₁−1, γ₁)` / `bit_unpack` | HIGH |
| `encoding::signature::serialize` / `deserialize` | `encoding::sig_encode` / `sig_decode` | HIGH |
| `encoding::signature::validate_hint_rows` / `write_hint_rows` (private) | A 2-pass refactor of `encoding::hint_bit_unpack` | MEDIUM |
| `polynomial::PolynomialRingElement::add` / `subtract` | `polynomial::poly_add` / `poly_sub` | HIGH |
| `polynomial::PolynomialRingElement::add_bounded` / `subtract_bounded` | `polynomial::poly_add` / `poly_sub` (with explicit bound ghosts) | MEDIUM |
| `polynomial::PolynomialRingElement::infinity_norm_exceeds` | `polynomial::poly_infinity_norm > bound` | HIGH |
| `polynomial::PolynomialRingElement::zero` | zero polynomial constant | HIGH |
| `hash_functions::shake256::*` / `shake128::*` | `hash_functions::*` (specs exist as opaque) | n/a (cryptographic black box) |

`hacspec_ml_dsa::keygen_internal` itself decomposes (in spec) into:
`expand_a`, `expand_s`, `vector_ntt(s1)`, `matrix_vector_ntt`,
`A·s₁`, `+ s₂`, `vector_intt`, `vector_power2round`, `pk_encode`,
`sk_encode`.  Every line in
`generate_key_pair`'s body has a 1-1 correspondence in
`keygen_internal`.  This makes the spec-equality `ensures` (already
wired) tractable as a future sprint after panic_free closes.

---

## Recommendations

1. **Chain 1 is bigger than the handoff estimated.** The bound on
   `sample_s1_and_s2` requires *also* surfacing bounds on
   `sample_four_error_ring_elements`,
   `sample_up_to_four_ring_elements_flat`, and the inner
   `rejection_sample_less_than_eta_*` private fns (or re-using their
   counter-only ensures).  The handoff's "1.5–2 sessions" estimate is
   plausible only if the agent chooses to *postulate* the bound on
   `sample_s1_and_s2` and discharge the body separately, not prove
   end-to-end.  Recommend the Class B handoff explicitly authorize
   that decomposition: surface the function-level `ensures` first,
   keep `admit ()` body, advance `generate_key_pair` past the NTT
   call, then circle back.  This matches the "Develop locally,
   upstream specs once" rule.

2. **Chain 2 collapses into Chain 1 + Chain 3 closure.**
   `verification_key::generate_serialized` and
   `signing_key::generate_serialized` are **already panic_free**;
   their preconditions match what Chain 1 (`s1_s2` η-bound) and Chain 3
   (`t0`/`t1` strict_lower bound) deliver.  The Class B Chain 2 owner's
   work is purely "thread the upstream ensures into the call site",
   not write new proofs.  Recommend folding Chain 2 into the Chain 3
   owner's plate (≤ 1 session).

3. **Chain 3 has a post-condition mismatch at
   `compute_as1_plus_s2`.** Its declared post is `is_i32b 16760832`
   (= 2·FIELD_MAX − 2), but `power2round_vector`'s pre is FIELD_MAX
   (= 8380416).  `2·FIELD_MAX > FIELD_MAX`, so the post does **not**
   directly chain.  The body in fact ends with a `reduce` + `+
   s1_s2[i]` (each ≤ η), so the actual output should be FIELD_MAX-bounded
   plus a small slack — likely the post should be **tightened** to
   FIELD_MAX (or a small multiple).  Recommend the Chain 3 agent
   inspect this first: either the post needs to drop to FIELD_MAX, or
   `generate_key_pair` needs an explicit `reduce`-before-power2round
   step, or `power2round_vector`'s pre needs to widen to `2·FIELD_MAX`
   (and propagate to `power2round`'s SIMD-trait pre).

4. **`samplex4::matrix_flat` needs a coefficient-bound ensures too.**
   The Class B handoff focuses on `s1_s2`, but
   `compute_as1_plus_s2`'s `a_as_ntt` precondition is
   `is_i32b_array_opaque FIELD_MAX` per coefficient.  Class A added
   only length-preservation.  The matrix `A` is rejection-sampled
   from `[0, Q)`, so the FIELD_MAX bound holds — the work is just
   surfacing it.  Recommend adding this to Chain 1's scope (sources
   the same trait-method opacity pattern as `sample_s1_and_s2`).

5. **`sample.rs` body-admits will not be discharged in this sprint.**
   5 functions are body-admitted, none touched by Chain 1's plan.
   Confirming the audit's interpretation of the handoff: keep them
   admitted; only surface the per-fn `ensures` that
   `compute_as1_plus_s2` / encoding need.  Worth flagging that the
   net body-admit count after Class B closes (8 → 5 in `sample.rs`,
   3 in `matrix.rs`, 2 in `arithmetic.rs`, 1 in `encoding/signature.rs`,
   9 in `ml_dsa_generic.rs`) is 20.  Closing keygen alone removes
   `generate_key_pair`'s admit and **3 dependencies**: 1 in
   `arithmetic.rs` (`power2round_vector`), 1 in `matrix.rs`
   (`compute_as1_plus_s2`), 0 in `sample.rs` (those stay admitted but
   gain `ensures`).  After this sprint: 17 body-admits remain.

6. **The signature `generate_serialized` precondition mismatch on
   `s1_2`.** `signing_key::generate_serialized` requires
   `is_pos_array_opaque η` on `s1_2` (line 29 of signing_key.rs:
   non-negative shifted form).  But the natural sample output is the
   centered `is_i32b η` form.  Either Chain 1's ensures must use
   `is_pos_array_opaque`, or there's a bridge lemma (an
   `is_i32b → is_pos` transform via `+η`).  Audit-flag: this
   shape mismatch should be the **first** thing the Chain 1 agent
   verifies before drafting the ensures.  If the bridge is missing,
   it needs to be added to `Spec.Utils` (cross-crate move — flag
   to parent).

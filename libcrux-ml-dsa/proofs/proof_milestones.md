# ML-DSA Proof Milestones

Hand-curated tracker of *meaningful* proof outcomes, complementing the
mechanical `verification_status.md` tier counts. Each row is a named
proof goal; status is judged by inspecting the owning function's
`#[ensures(...)]` text and body annotations on `ml-dsa-proofs`
(HEAD `c38294d8e` 2026-04-30 inventory pass).

## Status legend

- ✅ **Proven** — ensures cites `Hacspec_ml_dsa.*` (or `Spec.MLDSA.*`),
  body verifies (no admits).
- 🔶 **Spec stated, body admitted** — ensures shape correct but body
  has an admit.
- ⚠️ **Bounds only** — ensures proves a range/interval predicate; no
  hacspec equivalence.
- ❌ **No claim** — function has no ensures or is unverified at any tier.

## Gating classification (refresh 2026-04-30)

Every "many sprints" row in earlier revisions of this doc was
classified that way under the assumption the corresponding
`Hacspec_ml_dsa.*` spec did not exist. **The spec layer is in fact
substantial** (4318 LOC across 10 files, see "Existing spec inventory"
below). Each row is therefore re-classified as:

- ⛓️ **Wiring-gated** — Hacspec spec exists; the only remaining work is
  adding the `ensures` citation + composition proof through the layer
  beneath. No spec design needed.
- 📦 **Partial-spec / lemma-chain-gated** — top-level Hacspec spec
  exists, but the per-layer composition (e.g. SIMD chunking lemma,
  per-zeta layer-step lemma) needed to bridge `repr → spec` is not yet
  in place. Need ~1 lemma per layer/per width, then wire.
- 🆕 **Spec-gated** — no corresponding Hacspec definition. Need new
  spec module before any wiring.

After the inventory: of 25 milestone rows, **0 are 🆕 spec-gated**.
Almost everything is ⛓️ wiring-gated; NTT/Invntt rows are 📦
partial-spec because the per-layer-step commute lemmas (analogous to
ML-KEM's `lemma_inv_ntt_layer_<N>_step_to_hacspec`) are not yet
written. The "design Hacspec_ml_dsa.* from scratch" framing in the
prior milestones doc was incorrect.

## Existing spec inventory (2026-04-30)

`specs/ml-dsa/proofs/fstar/extraction/`:

| File | Lines | Defines |
|---|---|---|
| `Hacspec_ml_dsa.Ml_dsa.fst` | 634 | `keygen_internal`, `sign_internal`, `verify_internal` (the FIPS 204 algorithmic spec) |
| `Hacspec_ml_dsa.Ntt.fst` | 189 | FIPS 204 `v_ZETAS` table, `bit_rev_8_`, `ntt_layer`, `ntt`, `intt_layer`, `intt`, `reduce_polynomial` |
| `Hacspec_ml_dsa.Encoding.fst` | 1477 | `simple_bit_pack`, `bit_pack`, `simple_bit_unpack`, `bit_unpack`, `hint_bit_pack`, `hint_bit_unpack`, `pk_encode/decode`, `sk_encode/decode`, `sig_encode/decode`, `w1_encode`, `coeff_from_three_bytes`, `coeff_from_half_byte` |
| `Hacspec_ml_dsa.Polynomial.fst` | 434 | `poly_add/sub/pointwise_mul/reduce/mod_pm`, `vector_ntt/intt/add/sub`, `scalar_vector_ntt`, `vector_power2round/high_bits/low_bits/make_hint/use_hint`, `vector_infinity_norm`, `count_hints`, `poly_infinity_norm` |
| `Hacspec_ml_dsa.Sampling.fst` | 350 | `sample_in_ball`, `rej_ntt_poly`, `rej_bounded_poly`, `expand_a/s/mask`, byte-concat helpers |
| `Hacspec_ml_dsa.Arithmetic.fst` | 161 | `mod_q`, `mod_pm`, `power2round`, `decompose`, `high_bits/low_bits`, `make_hint`, `uuse_hint`, `coeff_norm`, `montgomery_reduce`, `shift_left_then_reduce` |
| `Hacspec_ml_dsa.Hash_functions.fst` | 85 | `h`, `g`, `h2`, `h3` (axiomatized SHAKE wrappers) |
| `Hacspec_ml_dsa.Matrix.fst` | 43 | `matrix_vector_ntt` (k×ℓ NTT-domain matrix-vector multiply) |
| `Hacspec_ml_dsa.Parameters.fst` | 183 | `t_MlDsaParams` record + `v_ZERO_POLY` |
| `Hacspec_ml_dsa.fst` | (umbrella) | `createi`, `t_Eta`, etc. |

`specs/ml-dsa/proofs/fstar/commute/`:

| File | Lines | Defines |
|---|---|---|
| `Hacspec_ml_dsa.Commute.Chunk.fst` | 762 | per-lane commute lemmas: `lemma_{reduce,add,sub,power2round,shift_left_then_reduce,decompose,use_hint,compute_hint,mont_mul,barrett_red}_lane_*` + bound/range helpers |

## Existing wiring (impl-side citations of Hacspec_ml_dsa)

Per-lane wiring (✅ proven and live in `ml-dsa-proofs`):

- `src/simd/traits/specs.rs` — per-lane post predicates citing
  `Hacspec_ml_dsa.Arithmetic.{decompose, uuse_hint, power2round,
  shift_left_then_reduce, make_hint}` and
  `Hacspec_ml_dsa.Encoding.{coeff_from_three_bytes, coeff_from_half_byte}`.
- `src/simd/avx2.rs` and `src/simd/portable.rs` — per-method bodies
  cite the matching `Hacspec_ml_dsa.Commute.Chunk.lemma_*_lane_commute*`
  bridges in their proof scripts.

NOT yet wired (these specs exist but no `ensures` cites them yet):

- `Hacspec_ml_dsa.Ntt.{ntt, intt, ntt_layer, intt_layer, reduce_polynomial}`.
- `Hacspec_ml_dsa.Encoding.{simple_bit_pack, bit_pack, simple_bit_unpack,
  bit_unpack, hint_bit_pack, hint_bit_unpack, pk_encode/decode,
  sk_encode/decode, sig_encode/decode, w1_encode}` — the **wrapper-level**
  encoding spec.
- `Hacspec_ml_dsa.Matrix.matrix_vector_ntt`.
- `Hacspec_ml_dsa.Polynomial.*` (vector + poly helpers).
- `Hacspec_ml_dsa.Sampling.*`.
- `Hacspec_ml_dsa.Hash_functions.{h, g, h2, h3}`.
- `Hacspec_ml_dsa.Ml_dsa.{keygen_internal, sign_internal, verify_internal}`
  — the FIPS 204 top-level spec.

---

## Layer 1 — NTT (forward & inverse)

The ml-dsa NTT operates on `PolynomialRingElement` of 32 SIMD units of
8 i32 lanes each (256 coefficients total). `Hacspec_ml_dsa.Ntt.ntt` /
`.intt` exist as 8-layer Cooley–Tukey functions over `t_Array i32 256`.
The wiring task is the chunking lemma converting `{32 simd_units, 8
lanes}` ↔ `{flat array of 256 i32}`, then per-layer-step commute
lemmas analogous to ML-KEM's `lemma_inv_ntt_layer_<N>_step_to_hacspec`.

| # | Milestone | Owner fn (file:line) | Status | Gating |
|---|---|---|---|---|
| 1 | Top-level forward NTT correct vs hacspec | `src/ntt.rs:15 ntt` | ⚠️ bounds only | 📦 partial-spec — `Hacspec_ml_dsa.Ntt.ntt` exists; need (a) chunking lemma simd_unit↔flat, (b) 8 per-layer commute lemmas in `Commute.Chunk`, (c) `lemma_ntt_full_commute` composition, (d) ensures citation. ~3 sprints |
| 2 | Top-level inverse NTT correct vs hacspec | `src/ntt.rs:28 invert_ntt_montgomery` | ⚠️ bounds only | 📦 partial-spec — `Hacspec_ml_dsa.Ntt.intt` exists; same shape as (1) plus Montgomery-domain conversion at exit. ~2-3 sprints after (1) |
| 3 | NTT multiply correct vs hacspec | `src/ntt.rs:57 ntt_multiply_montgomery` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Polynomial.poly_pointwise_mul` exists; per-lane `lemma_mont_mul_bound_and_mod_q` already in `Commute.Chunk`. ~1 sprint |
| 4 | Top-level Barrett reduce correct | `src/ntt.rs:44 reduce` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Arithmetic.mod_q` + per-lane `lemma_reduce_lane_commute` exist. ~1 sprint |
| 5 | Per-SIMD-unit NTT layers (Portable) | `src/simd/portable/ntt.rs::simd_unit_ntt_at_layer_{0,1,2}` | ⚠️ bounds only | 📦 partial-spec — same per-layer commute work as (1); these are the building blocks of (1) |
| 6 | Per-SIMD-unit NTT layers (AVX2) | `src/simd/avx2/ntt.rs::simd_unit_ntt_at_layer_{0,1,2}` | ⚠️ bounds only | same as (5) — AVX2 backend |
| 7 | Per-SIMD-unit inverse NTT (Portable & AVX2) | `src/simd/{portable,avx2}/invntt.rs` | ⚠️ bounds only | same as (5) for inverse |
| 8 | SIMD trait `ntt`/`invert_ntt` post | `src/simd/traits.rs` | ⚠️ bounds only via `is_i32b_array_opaque` | 📦 partial-spec — strengthen trait posts after (1)/(2) land |

## Layer 2 — Encoding / Serialization

ml-dsa has 8 encoding files for distinct field elements: `commitment`,
`error`, `gamma1`, `signature`, `signing_key`, `t0`, `t1`,
`verification_key`. Each is a (de)serialization of a fixed-width
field. **`Hacspec_ml_dsa.Encoding` already exists with the full FIPS
204 (de)serialization spec** (1477 lines): generic `simple_bit_pack`/
`bit_pack` + offset-encoded `bit_pack` + per-purpose `pk/sk/sig/w1`
encoders + `coeff_from_*` decoders.

The script-counted 'Hacspec' tier of 53 across the crate is
concentrated in `Simd/{Portable,Avx2}/Encoding/*` (the SIMD-level
helpers, not the high-level encoding) — those reference the per-lane
predicates. The wrapper layer is bounds-only.

| # | Milestone | Owner fn (file:line) | Status | Gating |
|---|---|---|---|---|
| 9 | `encoding/t0::serialize/deserialize` correct vs hacspec | `src/encoding/t0.rs:30, 88` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.bit_pack` (with offset 2^(d-1)) exists; need chunking lemma `Polynomial → t_Array i32 256` then ensures citation. ~1-2 sessions |
| 10 | `encoding/t1::serialize/deserialize` correct vs hacspec | `src/encoding/t1.rs` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.simple_bit_pack` (width 10). Same recipe as (9) with offsetless variant |
| 11 | `encoding/gamma1::serialize/deserialize` correct vs hacspec | `src/encoding/gamma1.rs` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.bit_pack` (offset = γ₁) |
| 12 | `encoding/error::serialize/deserialize` correct vs hacspec | `src/encoding/error.rs` | ⚠️ bounds only — `deserialize_to_vector_then_ntt` body closed `94e679871` (uniform `is_i32b 11` covers eta=2/4); `serialize` retains admit | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.bit_pack` (offset = η). Body unblock partial; remaining wire |
| 13 | `encoding/commitment::serialize/deserialize` correct vs hacspec | `src/encoding/commitment.rs` | ⚠️ bounds only | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.simple_bit_pack` (width = w1 max bits) or `w1_encode` |
| 14 | `encoding/signature::serialize/deserialize` correct vs hacspec | `src/encoding/signature.rs` | ⚠️ bounds only — `deserialize` body closed (PR 1348); `serialize` scaffolding (`count_total_ones` spec helper + Hamming-bound pre) landed `484fb664a`; body admit retained pending helper-split | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.sig_encode/sig_decode` exists. Per-fn ~1-2 sessions |
| 15 | `encoding/signing_key`, `verification_key` correct vs hacspec | resp. files | ⚠️ bounds only — `verification_key.{generate_serialized, deserialize}` body-closed; `signing_key.generate_serialized` body admit retained | ⛓️ wiring-gated — `Hacspec_ml_dsa.Encoding.sk_encode/decode, pk_encode/decode` exist |
| 16 | SIMD-backend per-encoding helpers | `src/simd/{portable,avx2}/encoding/{commitment,error,gamma1,t0,t1}.rs` | partial — 28 hacspec in Portable/encoding, 17 in Avx2/encoding (per script) | ⛓️ wiring-gated — these are the most advanced; the gap is in the wrapping layer (Generic) `encoding/*.rs` connecting them to the wrapper-level `Hacspec_ml_dsa.Encoding.*` spec |

## Layer 3 — Top-level ML-DSA API

`Hacspec_ml_dsa.Ml_dsa.{keygen_internal, sign_internal, verify_internal}`
already exist — these are the FIPS 204 algorithmic spec functions and
chain through `Hacspec_ml_dsa.{Hash_functions, Matrix, Polynomial,
Encoding, Sampling}`. Wiring them gives the strongest "this is FIPS
204" statement available without further spec work.

| # | Milestone | Owner fn (file:line) | Status | Gating |
|---|---|---|---|---|
| 17 | `MlDsa44::generate_key_pair` panic-free | `src/ml_dsa_44.rs:16, 183` | ❌ — no ensures shown; module sits in ADMIT_MODULES | ⛓️ wiring-gated — drops out of `ADMIT_MODULES` once `ml_dsa_generic.generate_key_pair_internal` body closes (row 21) |
| 18 | `MlDsa44::sign` correct vs hacspec | `src/ml_dsa_44.rs:38, 208` | ❌ — wraps `ml_dsa_generic.sign`; in `ADMIT_MODULES` | ⛓️ wiring-gated — gates on row 22 |
| 19 | `MlDsa44::verify` correct vs hacspec | `src/ml_dsa_44.rs:131, 266` | ❌ same | ⛓️ wiring-gated — gates on row 23 |
| 20 | `MlDsa65/87::*` (per-variant API) | `src/ml_dsa_65.rs`, `src/ml_dsa_87.rs` | ❌ same | ⛓️ wiring-gated — variants share `ml_dsa_generic` core |
| 21 | `ml_dsa_generic::generate_key_pair` correct vs `Hacspec_ml_dsa.Ml_dsa.keygen_internal` | `src/ml_dsa_generic.rs:57` | ❌ no ensures — body admit | ⛓️ wiring-gated — `Hacspec_ml_dsa.Ml_dsa.keygen_internal` exists. Wire ensures + compose. ~2-4 hr per fn (per agent prompt §1) |
| 22 | `ml_dsa_generic::sign_internal` correct vs `Hacspec_ml_dsa.Ml_dsa.sign_internal` | `src/ml_dsa_generic.rs:134` | ❌ — body admit (line 134 in audit table) | ⛓️ wiring-gated — `Hacspec_ml_dsa.Ml_dsa.sign_internal` exists. ~2-4 hr per fn |
| 23 | `ml_dsa_generic::verify_internal` correct vs `Hacspec_ml_dsa.Ml_dsa.verify_internal` | `src/ml_dsa_generic.rs:364` | ❌ — body admit (line 364) | ⛓️ wiring-gated — `Hacspec_ml_dsa.Ml_dsa.verify_internal` exists. ~2-4 hr per fn |
| 24 | `pre_hash` correct | `src/pre_hash.rs` | ⚠️ — included in Generic; check tier counts | ⛓️ wiring-gated — calls `Hacspec_ml_dsa.Hash_functions.h` |
| 25 | Hash functions correct (Shake128/256, Simd256/Portable/Neon backends) | `src/hash_functions.rs` (nested mods) | ⚠️ — ancestor-covered | ⛓️ wiring-gated — `Hacspec_ml_dsa.Hash_functions.{h, g, h2, h3}` are axiomatized; need length-preservation + per-call equivalence to the appropriate axiom |

## Headline interpretation

The script's "Hacspec: 53/577 (9.2%)" on ml-dsa is concentrated in:
- **Simd/Portable** (28 hacspec) — per-SIMD-unit encoding & arithmetic helpers
- **Simd/Avx2** (17 hacspec) — same, AVX2 backend
- **Generic** (8 hacspec) — scattered top-level lemmas

What the count does NOT reflect:
- The **Generic NTT** (rows 1-3) has zero hacspec — only bounds. The
  `Hacspec_ml_dsa.Ntt.{ntt, intt}` spec exists but is not yet cited;
  per-layer commute lemmas need to be authored before wiring.
- The **Generic encoding** (rows 9-15) has zero hacspec — only bounds
  + length preservation. The `Hacspec_ml_dsa.Encoding.*` spec exists;
  the wiring step (chunking lemma + ensures citation) has not been
  done for any wrapper.
- The **API surface** (rows 17-23) is mostly Lax (admitted) and has
  no top-level functional ensures. The `Hacspec_ml_dsa.Ml_dsa.*` spec
  exists; bodies of `ml_dsa_generic.{generate_key_pair, sign_internal,
  verify_internal}` are admitted.

## Comparison with ml-kem trajectory

The ml-dsa proof posture has a **stronger spec floor** than ml-kem-main
(the spec exists for everything, including the top-level `Ml_dsa`
function), but a **weaker wiring layer** (only per-lane primitives are
cited; no wrapper-level citations exist yet). The lift to
ml-kem-trait-opacify-level is a wiring-and-composition push, not a spec
design push.

| Aspect | ml-dsa (today) | ml-kem (main, pre-trait-opacify) | ml-kem (trait-opacify) |
|---|---|---|---|
| Per-lane Hacspec primitives wired | ✅ (Arithmetic + 2 Encoding) | partial | ✅ |
| Wrapper Hacspec wired (NTT, Encoding) | ❌ (specs exist, no citations) | partial | partial (4 inverse layers) |
| Top-level FIPS algorithmic spec exists | ✅ `Hacspec_ml_dsa.Ml_dsa.*` | (n/a — KEM not DSA) | (n/a) |
| Generic-orchestrator bodies closed | ❌ all 10 admit | (n/a) | (n/a) |
| `ADMIT_MODULES` discipline | allowlist (20 entries, 2026-04-30 flip) | allowlist | same |

## Next-priority order (refresh 2026-04-30)

Per agent prompt §0–§4, after this inventory:

1. **Wire `Ml_dsa_generic.{generate_key_pair_internal, sign_internal,
   verify_internal}` ensures to `Hacspec_ml_dsa.Ml_dsa.*`** (rows 21,
   22, 23). The Hacspec functions ALREADY EXIST; the wiring task is
   to add the `ensures` block citing the Hacspec function and close
   the body via composition. Per-fn ~2-4 hr. Closes rows 17-20 by
   propagation.
2. **Drive `ADMIT_MODULES` to zero** (parallel-friendly): cheapest
   unblocked groups are (a) Samplex4 (4 modules, ~2-3 hr per
   dispatcher) and (b) AVX2 Rejection_sample.{Less_than_eta,
   Less_than_field_modulus} (2 modules, ~1-2 hr each). Spec dispatcher
   for Sample.fst is one-step.
3. **Wire encoding wrappers** (rows 9–15). Per-wrapper
   `Hacspec_ml_dsa.Encoding.{simple_bit_pack, bit_pack, sk_encode,
   pk_encode, sig_encode, w1_encode}`-citation. Per-wrapper ~1-2
   sessions; 8 wrappers × ~12 hr total.
4. **Author NTT/Invntt per-layer commute lemmas in
   `Hacspec_ml_dsa.Commute.Chunk.fst`** (rows 1, 2, 5, 6, 7), then
   wire wrapper ensures (rows 1, 2). ~2-3 sprints across all layers +
   their inverses.

Not on the path: designing new `Hacspec_ml_dsa.*` spec modules.
Cross-audit AP-2 ("don't ensure-wire without verifying spec exists")
explicitly flagged that bounds-only ensures are sometimes a wiring
gap, sometimes a spec gap; for ML-DSA, after this inventory, the
answer for every row is **wiring gap**.
